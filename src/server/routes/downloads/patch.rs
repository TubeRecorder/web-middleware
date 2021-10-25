use log::trace;

use actix_web::{
  HttpRequest,
  HttpResponse,
  Responder,
};

use crate::errors::Error;

pub struct Patch {
  payload: Result<(), Error>,
}

impl Patch {
  pub fn new(payload: Result<(), Error>) -> Self {
    Self { payload }
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

    HttpResponse::Ok().body("successfully changed entry status")
  }
}
