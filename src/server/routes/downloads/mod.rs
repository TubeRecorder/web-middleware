use actix_web::{
  delete,
  get,
  patch,
  post,
  web,
  Responder,
};

use crate::{
  db::Entry,
  state::AppState,
};

use delete::Delete;
use get::Get;
use patch::Patch;
use post::Post;

mod delete;
mod get;
mod patch;
mod post;

#[post("/downloads")]
pub async fn post_downloads_handler(
  data: web::Data<AppState>,
  req_body: web::Json<Entry>,
) -> impl Responder {
  let payload = req_body.into_inner();

  Post::new(
    data
      .client
      .insert_download(payload)
      .await,
  )
}

#[patch("/downloads")]
pub async fn patch_downloads_handler(
  data: web::Data<AppState>,
  req_body: web::Json<(String, i32)>,
) -> impl Responder {
  let payload = req_body.into_inner();

  Patch::new(
    data
      .client
      .update_download_status(payload.0, payload.1)
      .await,
  )
}

#[delete("/downloads")]
pub async fn delete_downloads_handler(
  data: web::Data<AppState>,
  req_body: web::Json<String>,
) -> impl Responder {
  let payload = req_body.into_inner();

  Delete::new(
    data
      .client
      .delete_download(payload)
      .await,
  )
}

#[get("/downloads")]
pub async fn get_downloads_handler(
  data: web::Data<AppState>
) -> impl Responder {
  Get::new(data.client.get_downloads().await)
}
