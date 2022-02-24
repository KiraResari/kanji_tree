use iced::{Text, Font};

use crate::value_objects::Kanji;

pub struct KanjiPanel{
}

impl KanjiPanel{
    pub fn from(kanji: Kanji) -> Text{
        Text::new(
            kanji.character.to_string()
        ).size(64)
        .font(Font::External{
            name: "msgothic",
            bytes: include_bytes!("../fonts/msgothic.ttc")
        })
    }
}