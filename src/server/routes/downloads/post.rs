use log::trace;

use actix_web::{
  HttpRequest,
  HttpResponse,
  Responder,
};

use crate::errors::Error;

pub struct Post {
  payload: Result<(), Error>,
}

impl Post {
  pub fn new(payload: Result<(), Error>) -> Self {
    Self { payload }
  }
}

impl Responder for Post {
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

    HttpResponse::Ok().body("successfully created new entry")
  }
}
