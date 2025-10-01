use ::image::codecs::gif::GifDecoder;
use ::image::{AnimationDecoder, Frame};
use evdev::{Device, EventType};
use iced::border::Radius;
use iced::theme::Palette;
use iced::widget::{button, column, horizontal_space, row, text};
use iced::{Background, Border, Color, Length, Point, Theme};
use iced::{
    Element, Size, Subscription, Task,
    futures::{
        self, SinkExt, StreamExt,
        channel::{self, mpsc::Sender},
    },
    stream::channel,
    widget::{
        container,
        image::{self},
    },
    window::{Level, Position},
};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

struct AppState {
    cnt: u32,
    pressed: u16,
    frames: Vec<Frame>,
    frame_idx: usize,
    // playing: bool,
    // gif_path: PathBuf,
}

// type GifFrames = Vec<(Vec<u8>, u64, u16, u16)>;

impl Default for AppState {
    fn default() -> Self {
        // window::Action::Move(Id::unique(), Point { x: 1.0, y: 1000.0 });
        // let gif_path = PathBuf::from("/home/sun/Pictures/totoro-transparent.gif");
        let frames = AppState::load_gif();
        AppState {
            cnt: 0,
            pressed: 0,
            frames: frames,
            frame_idx: 0,
            // playing: true,
            // gif_path: gif_path,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Pressed(u16),
    NextFrame,
    OpenSetting,
}

impl AppState {
    fn update(state: &'_ mut AppState, message: Message) -> Task<Message> {
        match message {
            Message::Pressed(key) => {
                state.cnt += 1;
                state.pressed = key;

                Task::none()
            }
            Message::NextFrame => {
                let idx = (state.frame_idx + 1) % state.frames.len();
                state.frame_idx = idx;

                Task::none()
            }
            Message::OpenSetting => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            channel(1, |mut output: Sender<Message>| async move {
                let (tx, mut rx) = channel::mpsc::channel(10);

                std::thread::spawn(move || {
                    let path = Path::new("/dev/input/event8");
                    let mut device = Device::open(path).expect("Failed to open device");
                    println!(
                        "Listening for keyboard events on {}...",
                        device.name().unwrap_or("Unknown")
                    );

                    loop {
                        for ev in device.fetch_events().unwrap() {
                            if ev.event_type() == EventType::KEY && ev.value() == 1 {
                                futures::executor::block_on(
                                    tx.clone().send(Message::Pressed(ev.code())),
                                )
                                .unwrap();
                                futures::executor::block_on(tx.clone().send(Message::NextFrame))
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

    fn load_gif() -> Vec<Frame> {
        // let path = PathBuf::from("/home/sun/Pictures/bongo-cat-transparent.gif");
        let path = PathBuf::from("/home/sun/Downloads/keyboard-type-cat.gif");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let decoder = GifDecoder::new(reader).unwrap();
        let frames = decoder.into_frames().collect_frames().unwrap();

        // while let Some(frame) = decoder.read_next_frame().unwrap() {
        //     let rgba = frame.dispose;
        //     let dur_ms = frame.delay as u64 * 10;
        //     let w = frame.width;
        //     let h = frame.height;

        //     frames.push((rgba, dur_ms, w, h));
        // }
        println!("tlqlf");
        frames
    }

    fn window_settings() -> iced::window::Settings {
        iced::window::Settings {
            position: Position::Specific(Point::new(1000.0, 0.0)),
            size: Size::new(170.0, 190.0),
            transparent: true,
            decorations: false,
            resizable: false,
            level: Level::AlwaysOnTop,
            exit_on_close_request: false,
            ..Default::default()
        }
    }

    // fn settings() -> Settings {
    //     Settings {
    //         id:Some("asd"),
    //         fonts:font,

    //     }
    // }

    // fn new() -> Self {
    //     let path = PathBuf::from("/home/sun/Pictures/oiiaioiiiai.gif");
    //     (AppState::default())
    // }

    fn theme(&self) -> Theme {
        Theme::custom(
            "Custom theme".to_string(),
            Palette {
                background: Color::TRANSPARENT,
                ..Palette::DARK
            },
        )
    }

    fn view(state: &'_ AppState) -> Element<'_, Message> {
        let frames = &state.frames;
        let frame = &frames[state.frame_idx];
        // let delay = frame.delay().numer_denom_ms().0;
        let buffer = frame.buffer();
        let (w, h) = buffer.dimensions();
        let rgba = buffer.as_raw().clone();
        let handle = image::Handle::from_rgba(w, h, rgba);
        let k = format!(" {:?} ", state.cnt);

        let content = row![
            text(k)
                .size(20.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::BLACK)
                }),
            horizontal_space(),
            button("||||")
                .on_press(Message::OpenSetting)
                .style(|_theme, _status| button::Style {
                    background: Some(Background::Color(Color::from_rgb(160.0, 160.0, 160.0,))),
                    text_color: Color::BLACK,
                    ..Default::default()
                })
        ];

        column![
            image::Image::new(handle)
                .width(Length::Fixed(150.0))
                .height(Length::Fixed(150.0)),
            container(content).padding(2.0).style(|_| {
                iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        160.0, 160.0, 160.0,
                    ))),
                    // border: Border {
                    //     color: Color::from_rgb(10.0, 10.0, 20.0),
                    //     width: 10.0,
                    //     radius: Radius {
                    //         top_left: 0.2,
                    //         top_right: 0.2,
                    //         bottom_right: 0.2,
                    //         bottom_left: 0.2,
                    //     },
                    // },
                    ..Default::default()
                }
            })
        ]
        .into()
        // container(image::Image::new(tlqkf))
        //     .style(container::transparent)
        //     .into()
    }
}

fn main() {
    iced::application("keyboard", AppState::update, AppState::view)
        // .theme(|_s: &AppState| iced::Theme::)
        .subscription(AppState::subscription)
        .transparent(true)
        .window(AppState::window_settings())
        .theme(AppState::theme)
        .run_with(|| (AppState::default(), Task::none()))
        .unwrap()
}
