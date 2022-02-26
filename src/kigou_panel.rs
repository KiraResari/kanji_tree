use iced::{Text, Font, Column, Align, Row, Container};

use crate::{value_objects::Kigou, message::Message};

pub struct KigouPanel{
}

impl KigouPanel{
    pub fn from(kanji: &Kigou) -> Container<Message>{

        Container::new(
            Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new(kanji.stroke_count.to_string()))
                .push(KigouPanel::build_kanji_column(kanji))
                .push(Text::new(kanji.stroke_arrangement.to_string()))
        ).style(kanji.clone().kigou_type)
    }

    fn build_kanji_column(kanji: &Kigou) -> Column<Message>{
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(kanji.name.to_string()))
            .push(KigouPanel::build_kanji_character(kanji.character.clone()))
            .push(Text::new(kanji.kigou_type.to_string()))
    }

    fn build_kanji_character(character: String) -> Text {
        Text::new(character.to_string())
        .size(64)
        .font(Font::External{
            name: "msgothic",
            bytes: include_bytes!("../fonts/msgothic.ttc")
        })
    }
}