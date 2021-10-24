use serde::{
    Deserialize,
    Serialize,
};

use actix_web::{
    get,
    post,
    web,
    Responder,
};

use crate::state::AppState;

use get_downloads::GetDownloads;
use post_download::PostDownload;

mod get_downloads;
mod post_download;

#[get("/downloads")]
pub async fn get_downloads_handler(
    data: web::Data<AppState>
) -> impl Responder {
    GetDownloads::new(
        data.get_ref()
            .client
            .as_ref()
            .get_downloads()
            .await,
    )
}

#[derive(Serialize, Deserialize)]
pub struct PostDownloadPayload {
    pub url: String,
    pub local_path: String,
    pub status: i32,
}

#[post("/downloads")]
pub async fn post_downloads_handler(
    data: web::Data<AppState>,
    req_body: web::Json<PostDownloadPayload>,
) -> impl Responder {
    let payload = req_body.into_inner();

    PostDownload::new(
        data.get_ref()
            .client
            .as_ref()
            .insert_download((
                payload.url,
                payload.local_path,
                payload.status,
            ))
            .await,
    )
}
