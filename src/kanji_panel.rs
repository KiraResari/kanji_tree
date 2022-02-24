use iced::{Text, Font, Column, Align};

use crate::{value_objects::Kanji, message::Message};

pub struct KanjiPanel{
}

impl KanjiPanel{
    pub fn from(kanji: Kanji) -> Column<'static, Message>{
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(kanji.node_type.to_string()))
            .push(KanjiPanel::build_kanji_character(kanji.character))
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