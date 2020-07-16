use chrono::{ Date, Local };
use iced::{
    Application, Text, text_input, TextInput, button, Button, Settings, Column, Align, Element,
    Command, Scrollable, scrollable, Container, Length
};

fn main() {
    ChatBox::run(Settings::default());
}

// Single chat
#[derive(Debug)]
struct Chat {
    post_date: Date<Local>,
    text: String,
}

// Collected Chat
#[derive(Default)]
struct ChatBox {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    history: Vec<Chat>,
    post_button: button::State,
    clear_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    MessageChanged(String),
    MessagePosted,
    Cleared
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

                    self.history.push(new_post);
                    self.input_value.clear();
                }
            }
            Message::Cleared => {
                self.input_value.clear();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let chat_history = self.history
            .iter()
            .fold(
                Column::new().spacing(10),
                |column, chat| {
                    column
                        .push(
                            Column::new()
                                .push(Text::new(&chat.post_date.to_string()))
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&chat.text))
                        )
                }
            );

        let chat_box = Column::new()
            .padding(20)
            .align_items(Align::Start)
            .push(chat_history)
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
            .push(
                Button::new(&mut self.post_button, Text::new("Post"))
                    .on_press(Message::MessagePosted)
            )
            .push(
                Button::new(&mut self.clear_button, Text::new("Clear"))
                    .on_press(Message::Cleared)
            );

        Scrollable::new(&mut self.scroll)
            .padding(40)
            .push(
                Container::new(chat_box).width(Length::Fill).center_x(),
            )
            .into()
        
    }
}
