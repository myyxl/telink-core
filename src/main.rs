use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::http::server::start_webserver;

mod pcap;
mod http;

fn main() {
    let queue: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    pcap::receiver::start_packet_capture(queue.clone());
    start_webserver();
}