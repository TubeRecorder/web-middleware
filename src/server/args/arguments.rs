use std::{
    fmt::Debug,
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "web-middleware-server",
    about = "Web Middleware Server"
)]
pub struct Arguments {
    /// Activate debug mode
    #[structopt(long)]
    pub debug: bool,

    /// Service port number
    #[structopt(long, default_value = "8081")]
    pub service_port: i32,

    /// Activate stdout logging
    #[structopt(long)]
    pub stdout_log: bool,

    /// log file path
    #[structopt(long, parse(from_os_str))]
    pub log_file: Option<PathBuf>,

    /// Database type
    #[structopt(
        long,
        env = "DB_TYPE",
        default_value = "postgres"
    )]
    pub database_type: String,

    /// Database host
    #[structopt(
        long,
        env = "DB_HOST",
        default_value = "localhost"
    )]
    pub database_host: String,

    /// Database port
    #[structopt(
        long,
        env = "DB_PORT",
        default_value = "5432"
    )]
    pub database_port: i32,

    /// Database name
    #[structopt(long, env = "DB_NAME")]
    pub database_name: String,

    /// Database user name
    #[structopt(long, env = "DB_USER")]
    pub database_username: String,

    /// Database password
    #[structopt(long, env = "DB_PASSWORD")]
    pub database_password: String,

    /// Download service address
    #[structopt(
        long,
        env = "DOWNLOAD_HOST",
        default_value = "localhost"
    )]
    pub download_host: String,

    /// Download service port number
    #[structopt(
        long,
        env = "DOWNLOAD_PORT",
        default_value = "50051"
    )]
    pub download_port: i32,
}

impl Arguments {
    pub fn get() -> Self {
        Arguments::from_args()
    }
}
