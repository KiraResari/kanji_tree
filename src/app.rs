use iced::{Sandbox, Column, Element, Text, Font, Container, Length};

use crate::{kanji_parser::KanjiParser};

static KANJI_JSON_PATH: &str = "kanji.json";

pub struct KanjiTreeApp{
    active_kanji_name: String,
    active_kanji_character: String,
}

impl Sandbox for KanjiTreeApp {
    type Message = Message;

    fn new() -> KanjiTreeApp {
        let mut kanji_parser = KanjiParser::new();
        let kanji_source 
            = kanji_parser.parse_kanji_json(KANJI_JSON_PATH)
                .unwrap();
        let first_kanji_option = kanji_source.kanji.first();
        let first_kanji = first_kanji_option.unwrap();
        KanjiTreeApp{
            active_kanji_name: first_kanji.name.to_string(),
            active_kanji_character: first_kanji.character.to_string()
        }
    }

    fn title(&self) -> String {
        String::from("KanjiTree")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                
            }
            Message::DecrementPressed => {
                
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .push(
                Text::new(
                    self.active_kanji_character.to_string()
                ).size(64)
                .font(Font::External{
                    name: "msgothic",
                    bytes: include_bytes!("../fonts/msgothic.ttc")
                })
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}