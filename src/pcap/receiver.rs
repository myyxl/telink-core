use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use pcap::{Device, Capture, Linktype};

use crate::State;

pub fn start_packet_capture(state: Arc<Mutex<State>>) {
    thread::spawn(move || {
        let mut capture = Capture::from_device("wlp4s0mon")
            .unwrap()
            .immediate_mode(true)
            .open()
            .unwrap();

        capture.set_datalink(Linktype::IEEE802_11_RADIOTAP).unwrap();

        while let Ok(packet) = capture.next_packet() {
            match state.lock() {
                Ok(mut lock) => {
                    lock.queue.push_back(String::from(format!("{:?}", packet.header)));
                    lock.controller_last_ping = Some(SystemTime::now());
                }
                Err(_) => ()
            }
        }
        println!("After")
    });
}