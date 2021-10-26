use log::{
  error,
  info,
};
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::db::Client;

use download_api::{
  download_client::DownloadClient,
  DownloadRequest,
};

#[path = "../proto/download-api.rs"]
mod download_api;

pub async fn download_process(
  client: Arc<Client>,
  download_host: String,
  download_port: u16,
  rx: &mut mpsc::Receiver<String>,
) {
  while let Some(_) = rx.recv().await {
    info!("running download jobs");

    // let max_concurrent_downloads = client
    //   .get_max_download_connections()
    //   .await
    //   .unwrap()
    //   .unwrap();

    let downloads = client.get_downloads().await.unwrap();

    let addr = format!(
      "http://{}:{}",
      download_host.clone(),
      download_port
    );

    let end_point = match tonic::transport::Channel::from_shared(addr)
    {
      Ok(x) => x,
      Err(e) => {
        error!(
          "could not open end point: {}",
          e.to_string()
        );

        return;
      },
    };

    let channel = match end_point.connect().await {
      Ok(x) => x,
      Err(e) => {
        error!(
          "could not connect to channel: {}",
          e.to_string()
        );

        return;
      },
    };

    let mut client = DownloadClient::new(channel);

    for entry in &downloads {
      if entry.status == 0 {
        info!("skipping `{}`", entry.link_url);
        continue;
      }

      info!("downloading `{}`", entry.link_url);

      let request = tonic::Request::new(DownloadRequest {
        url: entry.link_url.clone(),
        local_path: entry.local_path.clone(),
      });

      // sending request and waiting for response
      match client.download_video(request).await {
        Ok(x) => x.into_inner(),
        Err(e) => {
          error!(
            "could not download link: {}",
            e.to_string()
          );

          continue;
        },
      };
    }
  }
}
