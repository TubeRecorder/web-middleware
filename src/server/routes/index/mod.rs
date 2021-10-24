use actix_web::{
    get,
    web,
    Responder,
};

use crate::state::AppState;

use get_index::GetIndex;

mod get_index;

#[get("/")]
pub async fn get_index_handler(
    data: web::Data<AppState>
) -> impl Responder {
    GetIndex::new(data.get_ref())
}
