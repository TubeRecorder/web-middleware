use log::trace;

use actix_web::{
  HttpRequest,
  HttpResponse,
  Responder,
};

use crate::{
  db::Entry,
  errors::Error,
};

pub struct Get {
  payload: Result<Vec<Entry>, Error>,
}

impl Get {
  pub fn new(payload: Result<Vec<Entry>, Error>) -> Self {
    Self { payload }
  }
}

impl Responder for Get {
  fn respond_to(
    self,
    req: &HttpRequest,
  ) -> HttpResponse {
    trace!("request {:?}", req);

    let downloads_list = match self.payload {
      Ok(x) => x,
      Err(e) => {
        return HttpResponse::ExpectationFailed()
          .body(format!("error: {}", e.to_string()));
      },
    };

    let payload = match serde_json::to_string(&downloads_list) {
      Ok(x) => x,
      Err(e) => {
        return HttpResponse::ExpectationFailed()
          .body(format!("error: {}", e.to_string()));
      },
    };

    HttpResponse::Ok().body(payload)
  }
}
