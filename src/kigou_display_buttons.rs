use iced::{Element, Text, Container, Image, Length};

use crate::{value_objects::Kigou, message::Message, fonts};

pub struct KigouDisplayButtons{
    kigou: Kigou,
}

impl KigouDisplayButtons{

    pub fn new(kigou: &Kigou) -> Self{
        KigouDisplayButtons {
            kigou: kigou.clone()
        }
    }

    pub fn view(&mut self) -> Element<Message>{
        if self.kigou.uses_image() {
            KigouDisplayButtons::build_kigou_image(
                self.kigou.clone(), 
            ).into()
        }else{
            KigouDisplayButtons::build_kigou_character(
                self.kigou.character.clone()
            ).into()
        }
    }

    fn build_kigou_character(
        character: String
    ) -> Text {
        Text::new(character.clone())
            .size(32)
            .font(fonts::KANJI)
    }

    fn build_kigou_image<'a>(kigou: Kigou) -> Container<'a, Message> {
        Container::new(
            Image::new(
                format!("resources/images/{}", kigou.image_name)
            )
            .width(Length::Units(32))
        )
    }
}