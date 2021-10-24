#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(
    clippy::pedantic,
    //missing_debug_implementations
)]

//! # web-middleware-service

use log::info;

// arguments
use args::Arguments;

// logger
use logs::setup_logger;

use actix_web::{
    web::Data,
    App,
    HttpServer,
};

use download::{
    download_video,
    index,
};
use state::AppState;

#[path = "../proto/download-api.rs"]
mod download_api;

// general modules
mod args;
mod errors;
mod logs;
mod state;

// api
mod download;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Box::new(Arguments::get());

    setup_logger(
        args.debug,
        args.stdout_log,
        args.log_file.clone(),
    )
    .unwrap();

    info!("{:?}", &args);

    let addr: String = format!("0.0.0.0:{}", args.service_port)
        .parse()
        .unwrap();

    info!("Server listening on {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                download_host: args.download_host.clone(),
                download_port: args.download_port,
            }))
            .service(index)
            .service(download_video)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
