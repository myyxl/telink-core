use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use crate::http::model::response::HttpResponse;
use crate::http::model::status::HttpStatus;

pub fn sse(stream: &mut TcpStream) -> Option<HttpResponse> {
    let eventstream_message = &HttpResponse {
        status: HttpStatus::Ok,
        body: String::new(),
        header: HashMap::from([
            (String::from("Content-Type"),String::from("text/event-stream")),
            (String::from("Cache-Control"),String::from("no-cache"))
        ])
    }.build();
    stream.write_all(eventstream_message).unwrap();

    loop {
        stream.write_all("data: UwU\n\n".as_bytes()).unwrap();
        thread::sleep(Duration::from_millis(100))
    }

}