use iced::alignment::Horizontal;
use iced::{widget::text, Application, Command, Element, Settings, Subscription};
use multi_thread_iced::controller::{self, SERIAL};
use multi_thread_iced::interface::DualShock4;
use tokio::sync::mpsc;
use std::cell::RefCell;

fn main() -> iced::Result {
    let mut dr = controller::DualShock4Driver::new(SERIAL).unwrap();
    let (sender, receiver) = mpsc::unbounded_channel::<DualShock4>();

    std::thread::spawn(move || {
        for _ in 0.. {
            let get = dr.task().unwrap();
            sender.send(get).unwrap();
        }
    });

    Ui::run(Settings::with_flags(UiFlags { receiver }))
}

struct UiFlags {
    receiver: mpsc::UnboundedReceiver<DualShock4>,
}

struct Ui {
    receiver: RefCell<Option<mpsc::UnboundedReceiver<DualShock4>>>,
    num: DualShock4,
}

#[derive(Debug, Clone)]
enum Message {
    ExternalMessageReceived(DualShock4),
}

impl Application for Ui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = UiFlags;

    fn new(flags: UiFlags) -> (Self, Command<Message>) {
        let app = Ui {
            receiver: RefCell::new(Some(flags.receiver)),
            num: DualShock4::new(),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("USAGI")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ExternalMessageReceived(num) => {
                self.num = num;
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::unfold(
            "led changes",
            self.receiver.take(),
            move |mut receiver| async move {
                let num = receiver.as_mut().unwrap().recv().await.unwrap();
                (Message::ExternalMessageReceived(num), receiver)
            },
        )
    }

    fn view(&self) -> Element<Message> {
        text(format!("x:{}\ny:{}\nrotation:{}", self.num.sticks.left_x, self.num.sticks.left_y, self.num.sticks.right_x)).horizontal_alignment(Horizontal::Center).size(50).font(iced::Font::MONOSPACE).into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::TokyoNight
    }
}