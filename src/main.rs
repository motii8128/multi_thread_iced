use iced::alignment::Horizontal;
use iced::{widget::{text, column, slider, combo_box, button}, Application, Command, Element, Settings, Subscription};
use multi_thread_iced::controller::{self, BLE, SERIAL};
use multi_thread_iced::interface::DualShock4;
use tokio::sync::mpsc;
use std::cell::RefCell;

fn main() -> iced::Result {
    
    let (sender, receiver) = mpsc::unbounded_channel::<DualShock4>();

    

    Ui::run(Settings::with_flags(UiFlags { receiver, sender }))
}

struct UiFlags {
    receiver: mpsc::UnboundedReceiver<DualShock4>,
    sender:mpsc::UnboundedSender<DualShock4>
}

struct Ui {
    sender: RefCell<Option<mpsc::UnboundedSender<DualShock4>>>,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<DualShock4>>>,
    num: DualShock4,
    rate:i32,
    controller_connection_types:iced::widget::combo_box::State<ConnectionType>,
    controller_connection_type:Option<ConnectionType>,
    type_:u8,
    state:State
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType
{
    Ble,
    Serial
}

#[derive(Debug, PartialEq)]
pub enum State
{
    ReadyNow,
    ControllerStarted
}

impl ConnectionType {
    const ALL:[ConnectionType;2]= [ConnectionType::Ble, ConnectionType::Serial];
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConnectionType::Ble=>"Ble",
                ConnectionType::Serial=>"Serial",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    ExternalMessageReceived(DualShock4),
    ScrollbarWidthChanged(u16),
    Connection(ConnectionType),
    OptionHovered(ConnectionType),
    ControllerStart
}




impl Application for Ui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = UiFlags;

    fn new(flags: UiFlags) -> (Self, Command<Message>) {
        let app = Ui {
            sender:RefCell::new(Some(flags.sender)),
            receiver: RefCell::new(Some(flags.receiver)),
            num: DualShock4::new(),
            rate:100,
            controller_connection_types:combo_box::State::new(ConnectionType::ALL.to_vec()),
            controller_connection_type:None,
            type_:0,
            state:State::ReadyNow
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
            Message::ScrollbarWidthChanged(n)=>{
                self.rate = n as i32;
            }
            Message::Connection(is_ble)=>{
                self.controller_connection_type = Some(is_ble);
                if is_ble == ConnectionType::Ble
                {
                    self.type_ = BLE;
                }
                else if is_ble == ConnectionType::Serial
                {
                    self.type_ = SERIAL
                }
            },
            Message::ControllerStart=>{
                let mut dr = controller::DualShock4Driver::new(self.type_).unwrap();
                let t = self.sender.clone().take().unwrap();
                std::thread::spawn(move || {
                    for _ in 0.. {
                        let get = dr.task();
                        t.clone().send(get).unwrap();
                    }
                });

                self.state = State::ControllerStarted;
            },
            Message::OptionHovered(is_ble)=>{
                println!("{}", is_ble)
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

    fn view(&self) -> Element<Message> 
    {
        if self.state == State::ReadyNow
        {
            let combo_box = combo_box(
                &self.controller_connection_types,
                "Select ControllerConnection ...",
                self.controller_connection_type.as_ref(),
                Message::Connection).on_option_hovered(Message::OptionHovered);
    
            let btn = button("Start Controller").on_press(Message::ControllerStart);

            let content = column![combo_box, btn].align_items(iced::alignment::Alignment::Start).spacing(50).into();

            content
        }   
        else if self.state == State::ControllerStarted
        {
            let sc = slider(
                0..=100,
                self.rate as u16,
                Message::ScrollbarWidthChanged,
            ).width(500);
    
            
    
            let controller_state = if self.num.state
            {
                "Connected!"
            }
            else 
            {
                "Not Connected"
            };        
            let r = self.rate as f32 /100.0;
            let lx = self.num.sticks.left_x * r;
            let ly = self.num.sticks.left_y * r;
            let rx = self.num.sticks.right_x * r;
            let text = text(format!("ControllerState:{}\nx:{}\ny:{}\nrotation:{}\nrate:{}%", controller_state, lx, ly, rx, self.rate)).horizontal_alignment(Horizontal::Center).size(50).font(iced::Font::MONOSPACE);
    
            let content = column![text, sc].align_items(iced::alignment::Alignment::Start).spacing(50).into();

            content
        }
        else {
            text("Error").into()
        }
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::TokyoNight
    }
}