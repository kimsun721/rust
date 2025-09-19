// use std::{ops::Sub, path::Path, process::Output, sync::mpsc::Sender};

// use evdev::{Device, EventType};
// use iced::{
//     Element, Subscription, Task,
//     widget::{button, column, image::Handle, text},
// };
// use iced_gif::{Frames, Gif};

// #[derive(Debug)]
// struct AppState {
//     cnt: u32,
//     pressed: u16,
// }

// // impl Default for AppState {
// //     fn default() -> Self {
// //         AppState { cnt: 0, pressed: 0 }
// //     }
// // }

// #[derive(Debug, Clone)]
// enum Message {
//     Pressed(u16),
// }

// fn update(state: &mut AppState, message: Message) -> Task<Message> {
//     match message {
//         Message::Pressed(key) => {
//             println!("123");
//             state.cnt += 1;
//             state.pressed = key;
//             println!("{:?}", key);
//             Task::none()
//         }
//     }
// }

// fn view(state: &AppState) -> Element<Message> {
//     // let gif_handle = Gif::new("/home/sun/Pictures/oiiaioiiiai.gif");
//     column![
//         text(format!("keyboard pressed {:?}", state)),
//         button("+").on_press(Message::Pressed(1)),
//     ]
//     .into()
// }

// fn subscription(state: &AppState) -> Subscription<Message> {
//     std::thread::spawn(move || {
//         let path = Path::new("/dev/input/event8");

//         let mut device = match Device::open(path) {
//             Ok(v) => v,
//             Err(_v) => panic!(),
//         };

//         println!(
//             "Listening for keyboard events on {}...",
//             device.name().unwrap_or("Unknown")
//         );

//         loop {
//             for ev in device.fetch_events().unwrap() {
//                 if ev.event_type() == EventType::KEY && ev.value() == 1 {
//                     Message::Pressed(ev.code());
//                 }
//             }
//         }
//     });
//     // if let Ok(v) = rx.try_recv() {
//     //     println!("gotgot{:?}", v);
//     // }
//     Subscription::none()
// }

// fn main() {
//     let mut state = AppState { cnt: 0, pressed: 0 };

//     iced::application("keyboard", update, view)
//         .theme(|_s| iced::Theme::Dark)
//         .subscription(subscription)
//         .run_with(|| (state, Task::none()));
// }

use std::path::{Path, PathBuf};

use evdev::{Device, EventType};
use iced::{
    Element, Subscription, Task,
    widget::{column, text},
};
use iced_gif::{Frames, gif};

#[derive(Debug)]
struct AppState {
    cnt: u32,
    pressed: u16,
    frames: Option<gif::Frames>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cnt: 10,
            pressed: 10,
            frames: None,
        }
    }
}

#[derive(Debug)]
enum Message {
    Pressed(u16),
    Loaded(Result<gif::Frames, gif::Error>),
}

impl AppState {
    fn update(state: &mut AppState, message: Message) -> Task<Message> {
        match message {
            Message::Pressed(key) => {
                state.cnt += 1;
                state.pressed = key;

                println!("{:?}qwdqwdkqopdodwkpoqwdopqkdwp", key);

                Task::none()
            }
            Message::Loaded(frames) => {
                state.frames = frames.ok();
                println!("{:?}qweqweqweqwee", state.frames);

                Task::none()
            }
        }
    }

    fn view(state: &AppState) -> Element<Message> {
        println!("{:?}", state);
        let gif1 = state.frames.as_ref();
        if let Some(v) = gif1 {
            column![gif(v), text(format!("keyboard pressed {:?}", state))].into()
        } else {
            column![
                text("Gif is loading..."),
                text(format!("keyboard pressed {:?}", state))
            ]
            .into()
        }
    }

    fn subscription(state: &AppState) -> Subscription<Message> {
        std::thread::spawn(move || {
            let path = Path::new("/dev/input/event8");

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
                        Message::Pressed(ev.code());
                    }
                }
            }
        });
        // if let Ok(v) = rx.try_recv() {
        //     println!("gotgot{:?}", v);
        // }
        Subscription::none()
    }

    fn new() -> (Self, Task<Message>) {
        let path = PathBuf::from("/home/sun/Pictures/oiiaioiiiai.gif");
        println!("{:?}", PathBuf::from("/home/sun/Pictures/oiiaioiiiai.gif"));
        (
            AppState::default(),
            gif::Frames::load_from_path(path).map(Message::Loaded),
        )
    }
}

fn main() {
    iced::application("keyboard", AppState::update, AppState::view)
        .theme(|_s| iced::Theme::Dark)
        .subscription(AppState::subscription)
        .run_with(AppState::new)
        .unwrap()
}
