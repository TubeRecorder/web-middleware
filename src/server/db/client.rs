use log::{
    debug,
    trace,
};

use sqlx::postgres::PgPool;

use crate::errors::Error;

use super::postgres_constants::*;

pub struct Client {
    pool: PgPool,
}

impl Client {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_download(
        &self,
        entry: (String, String, i32),
    ) -> Result<(), Error> {
        debug!("storing a new entry '{:?}'", entry);

        let sql = match 1 {
            1 => INSERT_DOWNLOAD,
            _ => UPDATE_DOWNLOAD,
        };

        match sqlx::query(sql)
            .bind(entry.2)
            .bind(&entry.0)
            .bind(&entry.1)
            .execute(&self.pool)
            .await
        {
            Ok(x) => {
                if x.rows_affected() != 1 {
                    return Err(Error::UnknownError(format!(
                        "insert/update query failed"
                    )));
                }
            },
            Err(e) => {
                return Err(Error::UnknownError(format!(
                    "unable to insert/update download entry with \
                     error: {}",
                    e
                )));
            },
        };

        Ok(())
    }

    pub async fn get_downloads(
        &self
    ) -> Result<Vec<(String, String, i32)>, Error> {
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
