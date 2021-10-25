use log::trace;

use actix_web::{
  HttpRequest,
  HttpResponse,
  Responder,
};

use crate::errors::Error;

pub struct GetDownloads {
  payload: Result<Vec<(String, String, i32)>, Error>,
}

impl GetDownloads {
  pub fn new(
    payload: Result<Vec<(String, String, i32)>, Error>
  ) -> Self {
    Self { payload }
  }
}

impl Responder for GetDownloads {
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
