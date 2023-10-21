use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct State {
    pub sse_connections: Vec<Arc<Mutex<TcpStream>>>,
    pub controller_ping: Option<Duration>,
}

impl Default for State {
    fn default() -> Self {
        State {
            sse_connections: Vec::new(),
            controller_ping: None
        }
    }
}