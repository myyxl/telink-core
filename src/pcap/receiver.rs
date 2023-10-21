use std::{process, thread};
use std::sync::mpsc::Sender;
use log::error;
use pcap::{Capture, Linktype};

pub fn start_packet_capture_thread(sender: Sender<String>, device: String) {
    thread::spawn(move || {
        let id = &generate_identifier("telink-core").to_be_bytes()[4..7];

        let mut capture = Capture::from_device(device.as_str())
            .unwrap()
            .immediate_mode(true)
            .open()
            .unwrap();

        capture.set_datalink(Linktype::IEEE802_11_RADIOTAP).unwrap();

        while let Ok(packet) = capture.next_packet() {

            // Check packet type
            if (packet.data[56] as usize) != 8 {
                continue;
            }

            // Check if core is the receiver
            let receiver = &packet.data[66..69];
            if receiver != id {
                continue;
            }

            // Read data
            let size: [u8; 2] = (&packet.data[80..82]).try_into().unwrap();
            let size: u16 = u16::from_be_bytes(size).into();
            let size: usize = size.into();
            let data = &packet.data[82..82 + size - 1];

            // Convert to &str
            let message = String::from_utf8_lossy(data);
            let message = message.trim();

            // Send to other thread
            sender.send(String::from(message)).unwrap_or_else(|error| {
                error!("{}", error);
                process::exit(0);
            });
        }
    });
}

fn generate_identifier(name: &str) -> u64 {
    let mut hash: u64 = 5381;
    for c in name.chars() {
        let c: u64 = c.into();
        hash = ((hash << 5) + hash) + c;
    }
    hash & 0xFFFFFFFF
}