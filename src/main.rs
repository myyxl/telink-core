use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use crate::http::server::start_webserver;

mod pcap;
mod http;
mod thread;

pub struct State {
    pub queue: VecDeque<String>,
    pub controller_last_ping: Option<SystemTime>,
}

fn main() {
    env_logger::init();
    let state = Arc::new(Mutex::new(
        State {
            queue: VecDeque::new(),
            controller_last_ping: None,
        }
    ));
    pcap::receiver::start_packet_capture(state.clone());
    start_webserver("127.0.0.1", 8000, state);
}