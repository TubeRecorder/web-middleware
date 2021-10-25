use log::trace;

use actix_web::{
  HttpRequest,
  HttpResponse,
  Responder,
};

use crate::errors::Error;

pub struct Patch {
  payload: Result<(), Error>,
  key: String,
}

impl Patch {
  pub fn new(
    payload: Result<(), Error>,
    key: String,
  ) -> Self {
    Self { payload, key }
  }
}

impl Responder for Patch {
  fn respond_to(
    self,
    req: &HttpRequest,
  ) -> HttpResponse {
    trace!("request {:?}", req);

    match self.payload {
      Ok(_) => {},
      Err(e) => {
        return HttpResponse::ExpectationFailed()
          .body(format!("error: {}", e.to_string()));
      },
    };

    HttpResponse::Ok().body(format!(
      "successfully changed configuration key `{}`",
      self.key
    ))
  }
}
