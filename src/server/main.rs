#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(
    clippy::pedantic,
    //missing_debug_implementations
)]

//! # web-middleware-service

use std::sync::Arc;

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

use db::{
  db_connection,
  Client,
};
use routes::*;
use state::AppState;

#[path = "../proto/download-api.rs"]
mod download_api;

// general modules
mod args;
mod db;
mod errors;
mod logs;
mod state;

// REST API routes
mod routes;

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

  let client = Arc::new(Client::new(
    db_connection(
      args.database_host.clone(),
      args.database_port,
      args.database_name.clone(),
      args.database_username.clone(),
      args.database_password.clone(),
    )
    .await
    .unwrap(),
  ));

  client
    .check_configs(
      args.max_concurrent_downloads,
      args.download_period_mins,
    )
    .await
    .unwrap();

  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(AppState::from(
        &args,
        client.clone(),
      )))
      // index
      .service(get_index_handler)
      // downloads
      .service(post_downloads_handler)
      .service(patch_downloads_handler)
      .service(delete_downloads_handler)
      .service(get_downloads_handler)
  })
  .bind(addr)?
  .run()
  .await?;

  Ok(())
}
