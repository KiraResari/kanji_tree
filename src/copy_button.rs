use iced::{button::State, Button, Text};

use crate::{message::Message, fonts};

pub struct CopyButton{
    state: State,
}

impl CopyButton{
    pub fn new() -> Self{
        CopyButton {state: State::new() }
    }

    pub fn view(&mut self) -> Button<Message> {
        Button::new(
            &mut self.state, 
            Text::new("🗐")
            .font(fonts::SYMBOL)
            .size(52)
        ).on_press(Message::CopyActiveKigouName())
    }
}