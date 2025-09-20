use std::path::{Path, PathBuf};

use evdev::{Device, EventType};
use iced::{
    Element, Subscription, Task,
    futures::{
        self, SinkExt, StreamExt,
        channel::{self, mpsc::Sender},
    },
    stream::channel,
    widget::{column, text},
};
use iced_gif::{Frames, gif};

#[derive(Debug)]
struct AppState {
    cnt: u32,
    pressed: u16,
    frames: Option<Frames>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cnt: 0,
            pressed: 0,
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
    fn update(state: &'_ mut AppState, message: Message) -> Task<Message> {
        match message {
            Message::Pressed(key) => {
                state.cnt += 1;
                state.pressed = key;

                Task::none()
            }
            Message::Loaded(frames) => {
                state.frames = frames.ok();

                Task::none()
            }
        }
    }

    fn view(state: &'_ AppState) -> Element<'_, Message> {
        if let Some(v) = &state.frames {
            column![
                gif(v),
                text(format!(
                    "keyboard pressed : {:?}, total : {:?}",
                    evdev::KeyCode::new(state.pressed),
                    state.cnt
                ))
            ]
            .into()
        } else {
            column![
                text("Gif is loading..."),
                text(format!(
                    "keyboard pressed : {:?}, total : {:?}",
                    evdev::KeyCode::new(state.pressed),
                    state.cnt
                ))
            ]
            .into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            channel(1, |mut output: Sender<Message>| async move {
                let (tx, mut rx) = channel::mpsc::channel(10);

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
                                Message::Pressed(ev.code());
                                futures::executor::block_on(
                                    tx.clone().send(Message::Pressed(ev.code())),
                                )
                                .unwrap();
                            }
                        }
                    }
                });
                loop {
                    if let Some(msg) = rx.next().await {
                        output.send(msg).await.unwrap();
                    }
                }
            })
        })
        // if let Ok(v) = rx.try_recv() {
        //     println!("gotgot{:?}", v);
        // }
    }

    fn new() -> (Self, Task<Message>) {
        let path = PathBuf::from("/home/sun/Pictures/oiiaioiiiai.gif");
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
