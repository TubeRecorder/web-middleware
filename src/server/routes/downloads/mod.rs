use actix_web::{
  delete,
  get,
  patch,
  post,
  web,
  HttpRequest,
  Responder,
};

use crate::{
  db::DownloadEntry,
  state::AppState,
};

use delete::Delete;
use get::Get;
use patch::Patch;
use post::Post;
use start::Start;

mod delete;
mod get;
mod patch;
mod post;
mod start;

#[post("/downloads")]
pub async fn post_downloads_handler(
  data: web::Data<AppState>,
  req: web::Json<DownloadEntry>,
) -> impl Responder {
  let payload = req.into_inner();

  Post::new(
    data
      .client
      .insert_download(payload)
      .await,
  )
}

#[patch("/downloads/{entry_id}/status/{status}")]
pub async fn patch_downloads_handler(
  data: web::Data<AppState>,
  req: HttpRequest,
) -> impl Responder {
  let entry_id: String = req
    .match_info()
    .get("entry_id")
    .unwrap()
    .parse()
    .unwrap();

  let status: i32 = req
    .match_info()
    .get("status")
    .unwrap()
    .parse()
    .unwrap();

  Patch::new(
    data
      .client
      .update_download_status(entry_id, status)
      .await,
  )
}

#[delete("/downloads/{entry_id}")]
pub async fn delete_downloads_handler(
  data: web::Data<AppState>,
  req: HttpRequest,
) -> impl Responder {
  let entry_id: String = req
    .match_info()
    .get("entry_id")
    .unwrap()
    .parse()
    .unwrap();

  Delete::new(
    data
      .client
      .delete_download(entry_id)
      .await,
  )
}

#[get("/downloads")]
pub async fn get_downloads_handler(
  data: web::Data<AppState>
) -> impl Responder {
  Get::new(data.client.get_downloads().await)
}

#[post("/downloads/start")]
pub async fn start_downloads_handler(
  data: web::Data<AppState>
) -> impl Responder {
  Start::new(
    data
      .tx
      .send(String::from("Requested"))
      .await,
  )
}
