use iced::{Text, Font, Column, Align, Row, Container, container};

use crate::{value_objects::Kanji, message::Message};

pub struct KanjiPanel{
}

impl KanjiPanel{
    pub fn from(kanji: &Kanji) -> Container<Message>{

        Container::new(
            Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new(kanji.stroke_count.to_string()))
                .push(KanjiPanel::build_kanji_column(kanji))
                .push(Text::new(kanji.stroke_arrangement.to_string()))
        ).style(Theme::Kanji)
    }

    fn build_kanji_column(kanji: &Kanji) -> Column<Message>{
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(kanji.name.to_string()))
            .push(KanjiPanel::build_kanji_character(kanji.character.clone()))
            .push(Text::new(kanji.node_type.to_string()))
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

pub enum Theme{
    Kanji,
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Kanji => kanji_theme::Container.into(),
        }
    }
}

mod kanji_theme {
    use iced::{container, Color};

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::BLACK),
                background: Color::from_rgb8(255, 127, 0).into(),
                border_color: Color::BLACK,
                border_width: 2.0,
                border_radius: 8.0,
            }
        }
    }
}