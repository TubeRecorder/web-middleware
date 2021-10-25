use log::trace;

use actix_web::{
  HttpRequest,
  HttpResponse,
  Responder,
};

use crate::{
  db::ConfigEntry,
  errors::Error,
};

pub struct Get {
  payload: Result<ConfigEntry, Error>,
}

impl Get {
  pub fn new(payload: Result<ConfigEntry, Error>) -> Self {
    Self { payload }
  }
}

impl Responder for Get {
  fn respond_to(
    self,
    req: &HttpRequest,
  ) -> HttpResponse {
    trace!("request {:?}", req);

    let entry = match self.payload {
      Ok(x) => x,
      Err(e) => {
        return HttpResponse::ExpectationFailed()
          .body(format!("error: {}", e.to_string()));
      },
    };

    let payload = match serde_json::to_string(&entry) {
      Ok(x) => x,
      Err(e) => {
        return HttpResponse::ExpectationFailed()
          .body(format!("error: {}", e.to_string()));
      },
    };

    HttpResponse::Ok().body(payload)
  }
}
