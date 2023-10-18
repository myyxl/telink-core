use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::http::model::request::HttpRequest;
use crate::http::model::response::HttpResponse;
use crate::http::model::status::HttpStatus;
use crate::http::routes::{service, telemetry};
use crate::State;

pub fn route(request: HttpRequest, stream: &mut TcpStream, state: Arc<Mutex<State>>) -> Option<HttpResponse> {
    match request.path.as_str() {
        "/service/status" => service::status(state),
        "/telemetry" => telemetry::sse(stream, state),
        _ => Some(HttpResponse::new()
            .status(HttpStatus::NotFound)
            .body("Not found"))
    }
}