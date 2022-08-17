use iced::{button::State, Button};

use crate::{value_objects::Kigou, message::Message, kigou_display_builder::KigouDisplayBuilder};

pub struct KigouButton{
    kigou: Kigou,
    state: State,
}

impl KigouButton{
    pub fn new(kigou: Kigou) -> Self{
        KigouButton { kigou, state: State::new() }
    }

    pub fn view(&mut self) -> Button<Message> {
        Button::new(
            &mut self.state, 
            KigouDisplayBuilder::build_kigou_display(&self.kigou, 32)
        ).on_press(Message::LoadKigou(self.kigou.clone()))
        .style(self.kigou.clone().kigou_type)
    }
}