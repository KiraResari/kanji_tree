use iced::{button::State, Button, Text, Font, Element, Container, Image, Length};

use crate::{value_objects::Kigou, message::Message};

pub struct KigouButton{
    kigou: Kigou,
    state: State,
}

impl KigouButton{
    pub fn new(kanji: Kigou) -> Self{
        KigouButton { kigou: kanji, state: State::new() }
    }

    pub fn view(&mut self) -> Button<Message> {
        Button::new(
            &mut self.state, 
            KigouButton::build_kigou_display(&self.kigou)
        ).on_press(Message::LoadKanji(self.kigou.clone()))
        .style(self.kigou.clone().kigou_type)
    }

    fn build_kigou_display(kigou: &Kigou) -> Element<Message>{
        if kigou.uses_image() {
            KigouButton::build_kigou_image(kigou).into()
        }else{
            KigouButton::build_kigou_character(kigou.character.clone()).into()
        }
    }

    fn build_kigou_character(character: String) -> Text {
        Text::new(character)
            .size(32)
            .font(Font::External{
                name: "msgothic",
                bytes: include_bytes!("../fonts/msgothic.ttc")
            })
    }

    fn build_kigou_image<'a>(kigou: &Kigou) -> Container<'a, Message> {
        Container::new(
            Image::new(
                format!("resources/images/{}", kigou.image_name)
            )
            .width(Length::Units(33))
        )
    }
}