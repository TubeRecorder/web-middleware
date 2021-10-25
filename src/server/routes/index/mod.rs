use actix_web::{
  get,
  web,
  Responder,
};

use crate::state::AppState;

use get::Get;

mod get;

#[get("/")]
pub async fn get_index_handler(
  data: web::Data<AppState>
) -> impl Responder {
  Get::new(data.get_ref())
}
