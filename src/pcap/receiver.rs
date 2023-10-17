use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_packet_capture(queue: Arc<Mutex<VecDeque<String>>>) {
    // Simulate packet capturing
    thread::spawn(move || {
        loop {
            queue.lock().unwrap().push_front(String::from("Testing"));
            thread::sleep(Duration::from_secs(3));
        }
    });
}