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

    /// Download service address
    #[structopt(long, default_value = "localhost")]
    pub download_host: String,

    /// Download service port number
    #[structopt(long, default_value = "50051")]
    pub download_port: i32,
}
