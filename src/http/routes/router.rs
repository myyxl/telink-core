use std::net::TcpStream;
use crate::http::model::request::HttpRequest;
use crate::http::model::response::HttpResponse;
use crate::http::model::status::HttpStatus;
use crate::http::routes::{service, telemetry};

pub fn route(request: HttpRequest, stream: &mut TcpStream) -> Option<HttpResponse> {
    match request.path.as_str() {
        "/service/status" => service::status(),
        "/telemetry" => telemetry::sse(stream),
        _ => Some(HttpResponse {
            status: HttpStatus::NotFound,
            body: String::from("Not found"),
            header: Default::default()
        })
    }
}