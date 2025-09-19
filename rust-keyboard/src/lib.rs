use std::path::Path;

use evdev::{Device, EventType, KeyCode};

pub fn key_logger(tx: std::sync::mpsc::Sender<u16>) {
    std::thread::spawn(move || {
        let path = Path::new("/dev/input/event20");

        let mut device = match Device::open(path) {
            Ok(v) => v,
            Err(_v) => panic!(),
        };

        println!(
            "Listening for keyboard events on {}...",
            device.name().unwrap_or("Unknown")
        );

        loop {
            for ev in device.fetch_events().unwrap() {
                if ev.event_type() == EventType::KEY && ev.value() == 1 {
                    let req = ev.code();
                    println!("{:?}", ev.code());

                    println!("{:?}", tx.send(req).unwrap());
                }
            }
        }
    });
}
