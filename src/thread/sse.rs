use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, RecvError};
use std::{process, thread};
use std::time::{SystemTime, UNIX_EPOCH};
use log::error;
use crate::State;

pub fn start(receiver: Receiver<String>, state: Arc<Mutex<State>>) {
    thread::spawn(move || {
        while let Ok(next) = &receiver.recv() {
            let mut state = state.lock().unwrap();
            state.controller_ping = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
            state.sse_connections.retain(|stream| {
                match stream.lock().unwrap().write_all(format!("data: {}\n\n", next).as_bytes()) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            });
        }
        error!("{}", RecvError);
        process::exit(0);
    });
}