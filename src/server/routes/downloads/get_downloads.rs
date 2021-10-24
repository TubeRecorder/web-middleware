use log::info;

use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
};

pub struct GetDownloads {
    downloads_list: Vec<(String, String, i32)>,
}

impl GetDownloads {
    pub fn new(downloads_list: Vec<(String, String, i32)>) -> Self {
        Self { downloads_list }
    }
}

impl Responder for GetDownloads {
    fn respond_to(
        self,
        _req: &HttpRequest,
    ) -> HttpResponse {
        info!("generating downloads list");

        let payload =
            match serde_json::to_string(&self.downloads_list) {
                Ok(x) => x,
                Err(e) => {
                    return HttpResponse::ExpectationFailed()
                        .body(format!("error: {}", e.to_string()))
                },
            };

        HttpResponse::Ok().body(payload)
    }
}
