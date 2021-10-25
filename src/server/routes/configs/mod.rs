use actix_web::{
  get,
  patch,
  web,
  HttpRequest,
  Responder,
};

use crate::{
  errors::Error,
  state::AppState,
};

use get::Get;
use patch::Patch;

mod get;
mod patch;

#[patch("/configs/{key}/{value}")]
pub async fn patch_configs_handler(
  data: web::Data<AppState>,
  req: HttpRequest,
) -> impl Responder {
  let key: String = req
    .match_info()
    .get("key")
    .unwrap()
    .parse()
    .unwrap();

  let payload = match key.as_str() {
    "max_download_connections" => {
      let value: u16 = req
        .match_info()
        .get("value")
        .unwrap()
        .parse()
        .unwrap();

      data
        .client
        .set_max_download_connections(value)
        .await
    },
    "download_period_mins" => {
      let value: u16 = req
        .match_info()
        .get("value")
        .unwrap()
        .parse()
        .unwrap();

      data
        .client
        .set_download_period_mins(value)
        .await
    },
    _ => {
      Err(Error::InvalidArgument(format!(
        "unknown configuration key `{}`",
        key
      )))
    },
  };

  Patch::new(payload, key)
}

#[get("/configs")]
pub async fn get_configs_handler(
  data: web::Data<AppState>
) -> impl Responder {
  Get::new(data.client.get_configs().await)
}
