use sqlx::postgres::{
    PgConnectOptions,
    PgPool,
    PgPoolOptions,
};

pub async fn db_connection(
    host: String,
    port_number: u16,
    database_name: String,
    username: String,
    password: String,
) -> Result<PgPool, sqlx::Error> {
    let options = PgConnectOptions::new()
        .host(host.as_str())
        .port(port_number)
        .database(database_name.as_str())
        .username(username.as_str())
        .password(password.as_str());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .unwrap();

    Ok(pool)
}
