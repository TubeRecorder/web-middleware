use log::info;
use serde::{
    Deserialize,
    Serialize,
};

use actix_web::{
    get,
    post,
    web,
    HttpResponse,
    Responder,
};

use crate::state::AppState;

use download_api::{
    download_client::DownloadClient,
    DownloadRequest,
};

#[path = "../../proto/download-api.rs"]
mod download_api;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub url: String,
    pub local_path: String,
}

#[post("/download_video")]
pub async fn download_video(
    data: web::Data<AppState>,
    req_body: String,
) -> impl Responder {
    info!("{:?}", req_body);

    let payload: Payload =
        match serde_json::from_str(req_body.as_str()) {
            Ok(x) => x,
            Err(e) => {
                return HttpResponse::BadRequest()
                    .body(e.to_string());
            },
        };

    let addr = format!(
        "http://{}:{}",
        data.download_host.clone(),
        data.download_port
    );

    let end_point = match tonic::transport::Channel::from_shared(addr)
    {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::PreconditionFailed()
                .body(e.to_string());
        },
    };

    let channel = match end_point.connect().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::PreconditionFailed()
                .body(e.to_string());
        },
    };

    let mut client = DownloadClient::new(channel);

    let request = tonic::Request::new(DownloadRequest {
        url: payload.url,
        local_path: payload.local_path,
    });

    // sending request and waiting for response
    let response = match client.download_video(request).await {
        Ok(x) => x.into_inner(),
        Err(e) => {
            return HttpResponse::PreconditionFailed()
                .body(e.to_string())
        },
    };

    HttpResponse::Ok().body(format!(
        "{{status: {}}}",
        response.status
    ))
}
