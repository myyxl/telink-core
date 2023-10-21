use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use log::debug;
use crate::http::model::request::HttpRequest;
use crate::http::model::response::HttpResponse;
use crate::http::model::status::HttpStatus;
use crate::http::routes::service;
use crate::State;

pub fn route(request: HttpRequest, stream: Arc<Mutex<TcpStream>>, state: Arc<Mutex<State>>) -> Option<HttpResponse> {
    match request.path.as_str() {
        "/service/status" => service::status(state),
        "/telemetry" => {
            init_sse(stream, state);
            None
        },
        _ => Some(HttpResponse::new()
            .status(HttpStatus::NotFound)
            .body("Not found"))
    }
}

fn init_sse(stream: Arc<Mutex<TcpStream>>, state: Arc<Mutex<State>>) {
    let eventstream = HttpResponse::new()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Access-Control-Allow-Origin", "*")
        .build();
    stream.lock().unwrap().write_all(&eventstream).unwrap();
    state.lock().unwrap().sse_receiver.push(stream);
    debug!("Current SSE connections: {}", state.lock().unwrap().sse_receiver.len());
}