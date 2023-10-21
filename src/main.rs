use std::collections::VecDeque;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::config::{Config, load_config};
use crate::http::server::start_webserver;

mod pcap;
mod http;
mod thread;
mod config;

pub struct State {
    pub queue: VecDeque<String>,
    pub sse_receiver: Vec<Arc<Mutex<TcpStream>>>,
    pub controller_last_ping: Option<Duration>,
    pub config: Config,
}

fn main() {
    env_logger::init();
    let config = load_config("config.toml");
    let state = Arc::new(Mutex::new(
        State {
            queue: VecDeque::new(),
            sse_receiver: Vec::new(),
            controller_last_ping: None,
            config
        }
    ));
    pcap::receiver::start_packet_capture(state.clone());
    start_webserver(state);
}