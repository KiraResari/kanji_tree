use iced::{button::State, Button, Text, Font};

use crate::{value_objects::Kigou, message::Message};

pub struct KigouButton{
    kanji: Kigou,
    state: State,
}

impl KigouButton{
    pub fn new(kanji: Kigou) -> Self{
        KigouButton { kanji: kanji, state: State::new() }
    }

    pub fn view(&mut self) -> Button<Message> {
        Button::new(
            &mut self.state, 
            Text::new(
                self.kanji.character.to_string()
            ).size(32)
            .font(Font::External{
                name: "msgothic",
                bytes: include_bytes!("../fonts/msgothic.ttc")
            }))
            .on_press(Message::LoadKanji(self.kanji.clone()))
            .style(self.kanji.clone().kigou_type)
    }
}