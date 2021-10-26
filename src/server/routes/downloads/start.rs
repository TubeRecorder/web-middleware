use log::trace;

use tokio::sync::mpsc::error::SendError;

use actix_web::{
  HttpRequest,
  HttpResponse,
  Responder,
};

pub struct Start {
  payload: Result<(), SendError<String>>,
}

impl Start {
  pub fn new(payload: Result<(), SendError<String>>) -> Self {
    Self { payload }
  }
}

impl Responder for Start {
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

    HttpResponse::Ok().body("successfully started downloads")
  }
}
