use std::path::Path;

use evdev::{Device, EventType};
use fltk::{
    app,
    frame::Frame,
    image::{AnimGifImage, AnimGifImageFlags},
    prelude::*,
    window::Window,
};

fn main() {
    std::thread::spawn(move || {
        let path = Path::new("/dev/input/event20");
        let mut device = Device::open(path).expect("Failed to open device");
        println!(
            "Listening for keyboard events on {}...",
            device.name().unwrap_or("Unknown")
        );

        loop {
            for ev in device.fetch_events().unwrap() {
                if ev.event_type() == EventType::KEY && ev.value() == 1 {
                    println!("{:?}", ev.code());
                }
            }
        }
    });

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 1000, 1000, "tlqkf");

    let mut frame = Frame::new(0, 0, 200, 200, "gif");

    let gif_path = Path::new("/home/sun/Downloads/keyboard-type-cat.gif");
    let mut gif = AnimGifImage::load(gif_path, &mut wind, AnimGifImageFlags::None).unwrap();
    gif.start();
    gif.set_speed(10.0);
    frame.set_image(Some(gif.clone()));

    wind.end();
    wind.show();

    app::add_idle3(move |_| {
        gif.next_frame();
        frame.redraw();
        app::sleep(0.1);
    });

    app.run().unwrap();
}
