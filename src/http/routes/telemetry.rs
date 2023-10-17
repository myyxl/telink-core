use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::http::model::response::HttpResponse;
use crate::State;

pub fn sse(stream: &mut TcpStream, state: Arc<Mutex<State>>) -> Option<HttpResponse> {
    let eventstream = HttpResponse::new()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .build();
    stream.write_all(&eventstream).unwrap();

    loop {
        let next = state.lock().unwrap().queue.pop_front();
        if let Some(next) = next {
            match stream.write_all(format!("data: {}\n\n", next).as_bytes()) {
                Err(_) => break,
                _ => ()
            }
        }
    }
    None
}