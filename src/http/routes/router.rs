use std::collections::VecDeque;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::http::model::request::HttpRequest;
use crate::http::model::response::HttpResponse;
use crate::http::model::status::HttpStatus;
use crate::http::routes::{service, telemetry};

pub fn route(request: HttpRequest, stream: &mut TcpStream, queue: Arc<Mutex<VecDeque<String>>>) -> Option<HttpResponse> {
    match request.path.as_str() {
        "/service/status" => service::status(),
        "/telemetry" => telemetry::sse(stream, queue),
        _ => Some(HttpResponse {
            status: HttpStatus::NotFound,
            body: String::from("Not found"),
            header: Default::default()
        })
    }
}