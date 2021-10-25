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

pub struct Client {
  pool: PgPool,
}

impl Client {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
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
    trace!("loading downloads list",);

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
  ) -> Result<i32, Error> {
    trace!("loading downloads list",);

    let rows: Vec<(String, String, i32)> =
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

    Ok(rows)
  }
}
