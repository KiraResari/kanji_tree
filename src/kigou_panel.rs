use iced::{Text, Font, Column, Align, Row, Container, Element, Image, Length};

use crate::{value_objects::Kigou, message::Message};

pub struct KigouPanel{
}

impl KigouPanel{
    pub fn from(kigou: &Kigou) -> Container<Message>{

        Container::new(
            Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new(kigou.stroke_count.to_string()))
                .push(KigouPanel::build_kigou_column(kigou))
                .push(Text::new(kigou.stroke_arrangement.to_string()))
        ).style(kigou.clone().kigou_type)
    }

    fn build_kigou_column(kigou: &Kigou) -> Column<Message>{
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(kigou.name.to_string()))
            .push(KigouPanel::build_kigou_display(kigou))
            .push(Text::new(kigou.kigou_type.to_string()))
    }

    fn build_kigou_display(kigou: &Kigou) -> Element<Message>{
        if kigou.uses_image() {
            KigouPanel::build_kigou_image(kigou).into()
        }else{
            KigouPanel::build_kigou_character(kigou.character.clone()).into()
        }
    }

    fn build_kigou_character(character: String) -> Text {
        Text::new(character)
            .size(64)
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
            .width(Length::Units(100))
        )
    }
}