use iced::{
    Application, Text, text_input, TextInput, button, Button, Settings, Column, Align, Element,
    Command, Container, Length, HorizontalAlignment, Row
};

pub fn display_ip_input() {
    IpInputBox::run(Settings::default());
}

#[derive(Debug, Default)]
struct IpInputBox {
    input: text_input::State,
    input_value: String,
    submit_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    InputIpState(Result<String, FormatError>),
    Submitted,
    Cleared,
}

#[derive(Debug, Clone)]
enum FormatError {
    IsEmpty,
    InvalidFormat
}

impl Application for IpInputBox {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (IpInputBox, Command<Message>) {
        (
            IpInputBox{..IpInputBox::default()},
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Simple Chat")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input_text) => {},
            Message::InputIpState(ip_addr) => {},
            Message::Submitted => {},
            Message::Cleared => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(
                TextInput::new(
                    &mut self.input,
                    "127.0.0.1",
                    &mut self.input_value,
                    Message::InputChanged,
                )
            )
            .push(
                Button::new(&mut self.submit_button,Text::new("Connect"))
                    .on_press(Message::Submitted)
            )
            .into()
    }
}
