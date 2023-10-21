use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::State;

pub fn start_sse_thread(state: Arc<Mutex<State>>) {
    thread::spawn(move || {
        loop {
            let mut state = state.lock().unwrap();
            if let Some(next) = state.queue.pop_front() {
                state.sse_receiver.retain(|stream| {
                    match stream.lock().unwrap().write_all(format!("data: {}\n\n", &next).as_bytes()) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                });
            }
        }
    });
}