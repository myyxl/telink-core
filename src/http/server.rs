use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use log::info;
use crate::http::model::request::HttpRequest;
use crate::http::routes::router;
use crate::thread::pool::ThreadPool;

pub fn start_webserver(host: &str, port: u16, queue: Arc<Mutex<VecDeque<String>>>) {
    let mut pool = ThreadPool::new(32);
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    info!("Started webserver on {}:{}", host, port);
    for stream in listener.incoming() {
        let queue = Arc::clone(&queue);
        let mut stream = stream.unwrap();
        pool.execute(move || {
            let request = parse_request(&mut stream);
            let response = router::route(request, &mut stream, queue);
            if let Some(response) = response {
                let bytes = &response.build();
                stream.write_all(bytes).unwrap()
            }
        });
    }
}


fn parse_request(stream: &mut TcpStream) -> HttpRequest {
    let raw_request: Vec<String> = BufReader::new(stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parts: Vec<&str> = raw_request[0].split(" ").collect();

    let mut header: HashMap<String, String> = HashMap::new();
    raw_request
        .iter()
        .skip(1)
        .map(|header| header.trim().split(":").collect())
        .for_each(|header_pair: Vec<&str>| {
            header.insert(String::from(header_pair[0]), String::from(header_pair[1]));
        });
    info!("{} {}", parts[0], parts[1]);
    HttpRequest {
        method: String::from(parts[0]),
        path: String::from(parts[1]),
        header,
    }
}