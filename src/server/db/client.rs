use sqlx::postgres::PgPool;

use super::postgres_constants::*;

pub struct Client {
    pool: PgPool,
}

impl Client {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_downloads(
        &self
    ) -> Result<Vec<(String, String, i32)>, sqlx::Error> {
        return Ok(vec![(
            String::from("1"),
            String::from("2"),
            3,
        )]);
    }
}
