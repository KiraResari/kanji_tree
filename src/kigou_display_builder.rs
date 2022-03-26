use iced::{Element, Text, Container, Image, Length};

use crate::{value_objects::Kigou, message::Message, fonts::{Fonts, self}};

pub struct KigouDisplayBuilder{
}

impl KigouDisplayBuilder{
    pub fn build_kigou_display(kigou: &Kigou, size: u16) -> Element<Message>{
        if kigou.uses_image() {
            KigouDisplayBuilder::build_kigou_image(
                kigou, 
                size
            ).into()
        }else{
            KigouDisplayBuilder::build_kigou_character(
                kigou.character.clone(),
                size
            ).into()
        }
    }

    fn build_kigou_character(character: String, size: u16) -> Text {
        Text::new(character)
            .size(size)
            .font(fonts::KANJI)
    }

    fn build_kigou_image<'a>(kigou: &Kigou, size: u16) -> Container<'a, Message> {
        Container::new(
            Image::new(
                format!("resources/images/{}", kigou.image_name)
            )
            .width(Length::Units(size))
        )
    }
}