use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::sync::{Arc, Mutex};
use log::{error, info};
use crate::http::model::error::HttpError;
use crate::http::model::method::get_request_methods;
use crate::http::model::request::HttpRequest;
use crate::http::routes::router;
use crate::State;
use crate::thread::pool::ThreadPool;

pub fn start_webserver(host: (String, u16), state: Arc<Mutex<State>>) {
    let mut pool = ThreadPool::new(32);
    let listener = TcpListener::bind(format!("{}:{}", host.0, host.1)).unwrap_or_else(|error| {
        error!("{}", error);
        process::exit(0);
    });
    info!("Started webserver on {}:{}", host.0, host.1);

    for stream in listener.incoming() {
        let state = Arc::clone(&state);
        let mut stream = stream.unwrap();
        pool.execute(move || {
            let Ok(request) = parse_request(&mut stream) else { return };

            let stream = Arc::new(Mutex::new(stream));
            let response = router::route(request, Arc::clone(&stream), state);

            if let Some(response) = response {
                let bytes = &response
                    .header("Access-Control-Allow-Origin", "*")
                    .build();
                stream.lock().unwrap().write_all(bytes).unwrap();
            }
        });
    }
}




fn parse_request(stream: &mut TcpStream) -> Result<HttpRequest, HttpError> {
    let raw_request: Vec<String> = BufReader::new(stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if raw_request.is_empty() {
        return Err(HttpError::RequestEmpty);
    }
    let parts: Vec<&str> = raw_request[0].split(" ").collect();
    let request_method = String::from(parts[0]);

    if !get_request_methods().contains(&request_method) {
        return Err(HttpError::UnknownRequestMethod);
    }
    if parts.len() == 1 {
        return Err(HttpError::MissingPath)
    }

    let mut header: HashMap<String, String> = HashMap::new();
    raw_request
        .iter()
        .skip(1)
        .map(|header| header.trim().split(":").collect())
        .for_each(|header_pair: Vec<&str>| {
            header.insert(String::from(header_pair[0]), String::from(header_pair[1]));
        });
    info!("{} {}", request_method, parts[1]);
    Ok(
        HttpRequest {
            method: request_method,
            path: String::from(parts[1]),
            header,
        }
    )
}