use log::trace;
use serde::Serialize;
use std::fmt::Debug;

use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
};

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct GetIndex {
    pub database_type: String,
    pub database_host: String,
    pub database_port: u16,
    pub database_name: String,
    pub download_host: String,
    pub download_port: u16,
}

impl GetIndex {
    pub fn new(app_state: &AppState) -> Self {
        Self {
            database_type: app_state.database_type.clone(),
            database_host: app_state.database_host.clone(),
            database_port: app_state.database_port,
            database_name: app_state.database_name.clone(),
            download_host: app_state.download_host.clone(),
            download_port: app_state.download_port,
        }
    }
}

impl Responder for GetIndex {
    fn respond_to(
        self,
        req: &HttpRequest,
    ) -> HttpResponse {
        trace!("request {:?}", req);

        let payload = match serde_json::to_string(&self) {
            Ok(x) => x,
            Err(e) => {
                return HttpResponse::ExpectationFailed()
                    .body(format!("error: {}", e.to_string()))
            },
        };

        HttpResponse::Ok().body(payload)
    }
}
