use iced::{button::State, Button, Text, Font};

use crate::{value_objects::Kanji, message::Message};

pub struct KanjiButton{
    kanji: Kanji,
    state: State,
}

impl KanjiButton{
    pub fn new(kanji: Kanji) -> Self{
        KanjiButton { kanji: kanji, state: State::new() }
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
    }
}