use log::{
  debug,
  trace,
};

use sqlx::postgres::PgPool;

use crate::errors::Error;

use super::{
  entry::Entry,
  postgres_constants::*,
};

static MAX_CONCURRENT_DOWNLOADS_KEY: &str =
  "MAX_CONCURRENT_DOWNLOADS";

static DOWNLOAD_PERIOD_MINS_KEY: &str = "DOWNLOAD_PERIOD_MINS";

pub struct Client {
  pool: PgPool,
}

impl Client {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn check_configs(
    &self,
    max_concurrent_downloads: u16,
    download_period_mins: u16,
  ) -> Result<(), Error> {
    debug!("checking for configurations");

    match self
      .get_max_download_connections()
      .await
      .unwrap()
    {
      Some(x) => {
        if max_concurrent_downloads != x {
          self
            .set_max_download_connections(max_concurrent_downloads)
            .await
            .unwrap();
        }
      },
      None => {
        self
          .set_max_download_connections_(
            max_concurrent_downloads,
            true,
          )
          .await
          .unwrap();
      },
    };

    match self
      .get_download_period_mins()
      .await
      .unwrap()
    {
      Some(x) => {
        if download_period_mins != x {
          self
            .set_download_period_mins(download_period_mins)
            .await
            .unwrap();
        }
      },
      None => {
        self
          .set_download_period_mins_(download_period_mins, true)
          .await
          .unwrap();
      },
    };

    Ok(())
  }

  pub async fn insert_download(
    &self,
    entry: Entry,
  ) -> Result<(), Error> {
    debug!("inserting a new entry '{:?}'", entry);

    match sqlx::query(INSERT_DOWNLOAD)
      .bind(&entry.entry_id)
      .bind(&entry.link_url)
      .bind(&entry.local_path)
      .bind(entry.status)
      .execute(&self.pool)
      .await
    {
      Ok(x) => {
        if x.rows_affected() != 1 {
          return Err(Error::UnknownError(format!(
            "insert query failed"
          )));
        }
      },
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to insert download entry, error: {}",
          e
        )));
      },
    };

    Ok(())
  }

  pub async fn update_download_status(
    &self,
    entry_id: String,
    status: i32,
  ) -> Result<(), Error> {
    debug!(
      "update an entry status: id :'{}', status: `{}`",
      entry_id, status
    );

    match sqlx::query(UPDATE_DOWNLOAD)
      .bind(status)
      .bind(entry_id)
      .execute(&self.pool)
      .await
    {
      Ok(x) => {
        if x.rows_affected() != 1 {
          return Err(Error::UnknownError(format!(
            "update state query failed"
          )));
        }
      },
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to update download entry status, error: {}",
          e
        )));
      },
    };

    Ok(())
  }

  pub async fn delete_download(
    &self,
    entry_id: String,
  ) -> Result<(), Error> {
    debug!("delete an entry: id :'{}'", entry_id);

    match sqlx::query(DELETE_DOWNLOAD)
      .bind(entry_id)
      .execute(&self.pool)
      .await
    {
      Ok(x) => {
        if x.rows_affected() != 1 {
          return Err(Error::UnknownError(format!(
            "delete entry query failed"
          )));
        }
      },
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to delete download entry, error: {}",
          e
        )));
      },
    };

    Ok(())
  }

  pub async fn get_downloads(&self) -> Result<Vec<Entry>, Error> {
    trace!("loading downloads list");

    let rows: Vec<(String, String, String, i32)> =
      match sqlx::query_as(SELECT_DOWNLOADS)
        .fetch_all(&self.pool)
        .await
      {
        Ok(x) => x,
        Err(e) => {
          return Err(Error::UnknownError(format!(
            "unable to load downloads table, error: {}",
            e,
          )));
        },
      };

    Ok(
      rows
        .iter()
        .map(|x| {
          Entry {
            entry_id: x.0.clone(),
            link_url: x.1.clone(),
            local_path: x.2.clone(),
            status: x.3,
          }
        })
        .collect(),
    )
  }

  pub async fn get_max_download_connections(
    &self
  ) -> Result<Option<u16>, Error> {
    trace!("loading maximum concurrent downloads");

    let rows: Vec<(String,)> = match sqlx::query_as(SELECT_CONFIG)
      .bind(MAX_CONCURRENT_DOWNLOADS_KEY)
      .fetch_all(&self.pool)
      .await
    {
      Ok(x) => x,
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to load configs table, error: {}",
          e,
        )));
      },
    };

    if rows.len() == 0 {
      return Ok(None);
    }

    let val: u16 = match serde_json::from_str(rows[0].0.as_str()) {
      Ok(x) => x,
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to deserialize maximum concurrent downloads, \
           error: {}",
          e,
        )));
      },
    };

    Ok(Some(val))
  }

  async fn set_max_download_connections_(
    &self,
    value: u16,
    is_insert: bool,
  ) -> Result<(), Error> {
    trace!("setting maximum concurrent downloads");

    let val = match serde_json::to_string(&value) {
      Ok(x) => x,
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to serialize maximum concurrent downloads \
           argument, error: {}",
          e
        )));
      },
    };

    let sql = match is_insert {
      true => INSERT_CONFIG,
      false => UPDATE_CONFIG,
    };

    match sqlx::query(sql)
      .bind(val)
      .bind(MAX_CONCURRENT_DOWNLOADS_KEY)
      .execute(&self.pool)
      .await
    {
      Ok(x) => {
        if x.rows_affected() != 1 {
          return Err(Error::UnknownError(format!(
            "insert maximum concurrent downloads configuration \
             failed"
          )));
        }
      },
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to insert maximum concurrent downloads \
           configuration, error: {}",
          e
        )));
      },
    };

    Ok(())
  }

  pub async fn set_max_download_connections(
    &self,
    value: u16,
  ) -> Result<(), Error> {
    self
      .set_max_download_connections_(value, false)
      .await
  }

  pub async fn get_download_period_mins(
    &self
  ) -> Result<Option<u16>, Error> {
    trace!("loading download period in mins");

    let rows: Vec<(String,)> = match sqlx::query_as(SELECT_CONFIG)
      .bind(DOWNLOAD_PERIOD_MINS_KEY)
      .fetch_all(&self.pool)
      .await
    {
      Ok(x) => x,
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to load configs table, error: {}",
          e,
        )));
      },
    };

    if rows.len() == 0 {
      return Ok(None);
    }

    let val: u16 = match serde_json::from_str(rows[0].0.as_str()) {
      Ok(x) => x,
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to deserialize download period in mins, error: {}",
          e,
        )));
      },
    };

    Ok(Some(val))
  }

  async fn set_download_period_mins_(
    &self,
    value: u16,
    is_insert: bool,
  ) -> Result<(), Error> {
    trace!("setting download period in mins");

    let val = match serde_json::to_string(&value) {
      Ok(x) => x,
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to serialize download period in mins argument, \
           error: {}",
          e
        )));
      },
    };

    let sql = match is_insert {
      true => INSERT_CONFIG,
      false => UPDATE_CONFIG,
    };

    match sqlx::query(sql)
      .bind(val)
      .bind(DOWNLOAD_PERIOD_MINS_KEY)
      .execute(&self.pool)
      .await
    {
      Ok(x) => {
        if x.rows_affected() != 1 {
          return Err(Error::UnknownError(format!(
            "insert download period in mins configuration failed"
          )));
        }
      },
      Err(e) => {
        return Err(Error::UnknownError(format!(
          "unable to insert download period in mins configuration, \
           error: {}",
          e
        )));
      },
    };

    Ok(())
  }

  pub async fn set_download_period_mins(
    &self,
    value: u16,
  ) -> Result<(), Error> {
    self
      .set_download_period_mins_(value, false)
      .await
  }
}
