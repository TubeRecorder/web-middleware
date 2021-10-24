use actix_web::{
    get,
    web,
    Responder,
};

use crate::state::AppState;

use get_downloads::GetDownloads;

mod get_downloads;

#[get("/downloads")]
pub async fn get_downloads_handler(
    data: web::Data<AppState>
) -> impl Responder {
    GetDownloads::new(
        data.get_ref()
            .client
            .as_ref()
            .get_downloads()
            .await
            .unwrap(),
    )
}
