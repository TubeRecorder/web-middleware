use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
  args::Arguments,
  db::Client,
};

pub struct AppState {
  pub database_type: String,
  pub database_host: String,
  pub database_port: u16,
  pub database_name: String,
  pub database_username: String,
  pub database_password: String,

  pub download_host: String,
  pub download_port: u16,

  pub client: Arc<Client>,

  pub tx: mpsc::Sender<String>,
}

impl AppState {
  pub fn from(
    args: &Arguments,
    client: Arc<Client>,
    tx: mpsc::Sender<String>,
  ) -> Self {
    let x = Self {
      database_type: args.database_type.clone(),
      database_host: args.database_host.clone(),
      database_port: args.database_port,
      database_name: args.database_name.clone(),
      database_username: args.database_username.clone(),
      database_password: args.database_password.clone(),
      download_host: args.download_host.clone(),
      download_port: args.download_port,
      client,
      tx,
    };

    x
  }
}
