use iced::{widget::text, Application, Command, Element, Settings, Subscription};
use tokio::sync::mpsc;
use std::cell::RefCell;

fn main() -> iced::Result {
    let (sender, receiver) = mpsc::unbounded_channel::<i32>();

    std::thread::spawn(move || {
        for i in 0.. {
            sender.send(i).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    Ui::run(Settings::with_flags(UiFlags { receiver }))
}

struct UiFlags {
    receiver: mpsc::UnboundedReceiver<i32>,
}

struct Ui {
    receiver: RefCell<Option<mpsc::UnboundedReceiver<i32>>>,
    num: i32,
}

#[derive(Debug, Clone)]
enum Message {
    ExternalMessageReceived(i32),
}

impl Application for Ui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = UiFlags;

    fn new(flags: UiFlags) -> (Self, Command<Message>) {
        let app = Ui {
            receiver: RefCell::new(Some(flags.receiver)),
            num: 0,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("External Message Example")
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
        text(self.num).into()
    }
}