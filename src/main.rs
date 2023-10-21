use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use crate::config::load_config;
use crate::http::server::start_webserver;
use crate::state::State;

mod http;
mod thread;
mod config;
mod state;

fn main() {
    env_logger::init();

    let config = load_config("config.toml");
    let state = Arc::new(Mutex::new(State::default()));
    let (sender, receiver) = channel();

    thread::pcap::start(sender, config.monitor_interface);
    thread::sse::start(receiver, Arc::clone(&state));
    start_webserver((config.host, config.port), Arc::clone(&state));
}