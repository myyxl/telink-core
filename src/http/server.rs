use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use crate::http::model::request::HttpRequest;
use crate::http::model::response::HttpResponse;
use crate::http::routes::service;

pub fn start_webserver() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let raw_request: Vec<String> = BufReader::new(&mut stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        let request = parse_request(raw_request);
        let response = match request.path.as_str() {
            "/service/status" => service::status(),
            _ => HttpResponse {
                status: 404,
                body: String::from("Not found"),
                header: Default::default()
            }
        };

        let bytes = &build_response(response);
        stream.write_all(bytes).unwrap()
    }
}


fn parse_request(raw_request: Vec<String>) -> HttpRequest {
    let parts: Vec<&str> = raw_request[0].split(" ").collect();

    let mut header: HashMap<String, String> = HashMap::new();
    raw_request
        .iter()
        .skip(1)
        .map(|header| header.trim().split(":").collect())
        .for_each(|header_pair: Vec<&str>| {
            header.insert(String::from(header_pair[0]), String::from(header_pair[1]));
        });
    HttpRequest {
        method: String::from(parts[0]),
        path: String::from(parts[1]),
        header,
    }
}

fn build_response(response: HttpResponse) -> Box<[u8]> {
    Box::from(
        format!("HTTP/1.1 {} OK\r\n\r\n{}", response.status, response.body).as_bytes()
    )
}