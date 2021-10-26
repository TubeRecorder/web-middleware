use log::info;
use std::sync::Arc;

use tokio::{
  sync::mpsc,
  time::{
    sleep,
    Duration,
  },
};

use crate::db::Client;

pub async fn timer_process(
  client: Arc<Client>,
  tx: mpsc::Sender<String>,
) {
  loop {
    info!("timer expired");

    // send signal
    tx.send(String::from("timer expired"))
      .await
      .unwrap();

    // sleep

    let download_period_mins = client
      .get_download_period_mins()
      .await
      .unwrap()
      .unwrap();

    sleep(Duration::from_secs(
      (download_period_mins * 60) as u64,
    ))
    .await;
  }
}
