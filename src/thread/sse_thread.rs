use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::State;

pub fn start_sse_thread(state: Arc<Mutex<State>>) {
    thread::spawn(move || {
        loop {
            let mut state = state.lock().unwrap();
            if let Some(next) = state.queue.pop_front() {
                let mut offset = 0;
                for(index, stream) in state.sse_receiver.clone().iter().enumerate() {
                    match stream.lock().unwrap().write_all(format!("data: {}\n\n", &next).as_bytes()) {
                        Err(_) => {
                            state.sse_receiver.remove(index - offset);
                            offset = offset + 1;
                        },
                        _ => {}
                    }
                }

            }
        }
    });
}