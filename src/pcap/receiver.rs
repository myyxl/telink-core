use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use crate::State;

pub fn start_packet_capture(state: Arc<Mutex<State>>) {
    // Simulate packet capturing
    thread::spawn(move || {
        loop {
            match state.lock() {
                Ok(mut lock) => {
                    lock.queue.push_front(String::from("Testing"));
                    lock.controller_last_ping = Some(SystemTime::now());
                }
                Err(_) => ()
            }
            thread::sleep(Duration::from_secs(3));
        }
    });
}