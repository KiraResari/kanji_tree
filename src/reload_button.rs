use iced::{button::State, Button, Text};

use crate::{message::Message};

pub struct ReloadButton{
    state: State,
}

impl ReloadButton{
    pub fn new() -> Self{
        ReloadButton { state: State::new() }
    }

    pub fn view(&mut self) -> Button<Message> {
        Button::new(
            &mut self.state, 
            Text::new("Reload kanji.json")
        ).on_press(Message::ReloadKigouSource())
    }
}