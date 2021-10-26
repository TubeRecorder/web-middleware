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
use std::sync::Arc;

use tokio::sync::mpsc;

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
use download_thread::download_process;
use routes::*;
use state::AppState;
use timer_thread::timer_process;

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

// threads
mod download_thread;
mod timer_thread;

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

  let (tx, mut rx) = mpsc::channel(2);
  let tx2: mpsc::Sender<String> = tx.clone();

  {
    let client0 = client.clone();
    let download_host = args.download_host.clone();
    let download_port = args.download_port;

    tokio::spawn(async move {
      download_process(
        client0,
        download_host,
        download_port,
        &mut rx,
      )
      .await;
    });
  }

  {
    let client0 = client.clone();

    tokio::spawn(async move {
      timer_process(client0, tx2).await;
    });
  }

  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(AppState::from(
        &args,
        client.clone(),
        tx.clone(),
      )))
      // index
      .service(get_index_handler)
      // configs
      .service(patch_configs_handler)
      .service(get_configs_handler)
      // downloads
      .service(post_downloads_handler)
      .service(patch_downloads_handler)
      .service(delete_downloads_handler)
      .service(get_downloads_handler)
      .service(start_downloads_handler)
  })
  .bind(addr)?
  .run()
  .await?;

  info!("Server shutting down");

  Ok(())
}
