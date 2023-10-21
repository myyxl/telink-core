use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use crate::config::load_config;
use crate::http::server::start_webserver;
use crate::pcap::receiver::start_packet_capture_thread;
use crate::state::State;
use crate::thread::sse_thread::start_sse_thread;

mod pcap;
mod http;
mod thread;
mod config;
mod state;

fn main() {
    env_logger::init();

    let config = load_config("config.toml");
    let state = Arc::new(Mutex::new(State::default()));
    let (sender, receiver) = channel();

    start_packet_capture_thread(sender, config.monitor_interface);
    start_sse_thread(receiver, Arc::clone(&state));
    start_webserver((config.host, config.port), Arc::clone(&state));
}