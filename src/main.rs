use chrono::{ Date, Local };
use iced::{
    Application, Text, text_input, TextInput, button, Button, Settings, Column, Align, Element,
    Command, Scrollable, scrollable, Container, Length, HorizontalAlignment, Row, Rectangle
};
mod p2p_node;

fn main() {
    ChatBox::run(Settings::default());
}

// Chat box
#[derive(Default)]
struct ChatBox {
    input: text_input::State,
    input_value: String,
    input_ip: text_input::State,
    input_ip_value: String,
    chat_history: ChatHistory,
    post_button: button::State,
    clear_button: button::State,
    connect_button: button::State,
}

// Chat history
#[derive(Debug, Default)]
struct ChatHistory{
    scroll: scrollable::State,
    chats: Vec<Chat>,
}

// Single chat
#[derive(Debug)]
struct Chat {
    post_date: Date<Local>,
    text: String,
}

#[derive(Debug, Clone)]
enum Message {
    MessageChanged(String),
    MessagePosted,
    Cleared,
    IpStringInput(String),
    Connecting,
    Connected,
}

impl Application for ChatBox {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ChatBox, Command<Message>) {
        (
            ChatBox{..ChatBox::default()},
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Simple Chat")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MessageChanged(text) => {
                self.input_value = text;
            }
            Message::MessagePosted => {
                if !self.input_value.is_empty() {
                    let post_date = Local::today();
                    let post_message = &self.input_value;

                    let new_post = Chat {
                        post_date: post_date,
                        text: post_message.to_string(),
                    };

                    self.chat_history.chats.push(new_post);
                    self.input_value.clear();
                }
            }
            Message::Cleared => {
                self.input_value.clear();
            },
            Message::IpStringInput(input_addr) => {
                self.input_ip_value = input_addr;
            },
            Message::Connecting => {
                p2p_node::subscribe_swarm(self.input_ip_value.to_string());
            },
            Message::Connected => {},
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let header_title = Text::new("Simple Chat")
            .horizontal_alignment(HorizontalAlignment::Left)
            .size(30);

        let chat_history = self.chat_history.chats
            .iter()
            .fold(
                Column::new().spacing(4).align_items(Align::Start).width(Length::Fill).padding(5),
                |column, chat| {
                    column
                        .push(
                            Column::new()
                                .push(
                                    Text::new(&chat.post_date.to_string())
                                        .horizontal_alignment(HorizontalAlignment::Left)
                                )
                        )
                        .push(
                            Column::new()
                                .push(
                                    Text::new(&chat.text)
                                        .horizontal_alignment(HorizontalAlignment::Left)
                                        .size(30)
                                )
                        )
                }
            );

        let scrollable_history = Scrollable::new(&mut self.chat_history.scroll)
            .padding(40)
            .height(Length::FillPortion(4))
            .push(
                Container::new(chat_history).width(Length::Fill).center_x(),
            );

        let operation_buttons = Column::new()
            .padding(5)
            .spacing(5)
            .align_items(Align::End)
            .width(Length::Fill)
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        Button::new(&mut self.post_button, Text::new("Post"))
                            .on_press(Message::MessagePosted)
                    )
                    .push(
                        Button::new(&mut self.clear_button, Text::new("Clear"))
                            .on_press(Message::Cleared)
                    )
            );

        let connect_setting = Column::new()
            .padding(5)
            .width(Length::Fill)
            .align_items(Align::End)
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        TextInput::new(
                            &mut self.input_ip,
                            "127.0.0.1",
                            &self.input_ip_value,
                            Message::IpStringInput,
                        )
                            .width(Length::Units(150))
                            .padding(10)
                            .on_submit(Message::Connecting)
                    )
                    .push(
                        Button::new(&mut self.connect_button, Text::new("Connect"))
                            .padding(10)
                            .on_press(Message::Connecting)
                    )
            );

        Column::new()
            .padding(20)
            .push(header_title)
            .push(scrollable_history)
            .push(connect_setting)
            .push(
                TextInput::new(
                    &mut self.input,
                    "Hello :)",
                    &self.input_value,
                    Message::MessageChanged,
                )
                    .padding(15)
                    .size(30)
                    .on_submit(Message::MessagePosted)
            )
            .push(operation_buttons)
            .into()
    }
}
