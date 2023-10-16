use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::http::server::start_webserver;

mod pcap;
mod http;

fn main() {
    env_logger::init();
    let queue: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    pcap::receiver::start_packet_capture(queue.clone());
    start_webserver("127.0.0.1", 8000);
}