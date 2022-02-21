use iced::{Sandbox, Column, Element, Text, Font, Container, Length};

use crate::{kanji_parser::KanjiParser, kanji_source::KanjiSource, value_objects::Kanji};

static KANJI_JSON_PATH: &str = "kanji.json";

pub struct KanjiTreeApp{
    kanji_source: KanjiSource,
    active_kanji: Kanji,
}

impl KanjiTreeApp{
    fn load_first_kanji(kanji_source: &KanjiSource) -> Kanji{
        let first_kanji_result
             = kanji_source.get_first_element();
        match first_kanji_result{
            Ok(v) => v,
            Err(e) => Kanji::create_error_kanji(&e.to_string())
        }
    }
}

impl Sandbox for KanjiTreeApp {
    type Message = Message;

    fn new() -> KanjiTreeApp {
        let mut kanji_parser = KanjiParser::new();
        let kanji_source 
            = kanji_parser.parse_kanji_json(KANJI_JSON_PATH)
                .unwrap();
        let active_kanji
            = KanjiTreeApp::load_first_kanji(&kanji_source);
        KanjiTreeApp{
            kanji_source,
            active_kanji
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
                    self.active_kanji.character.to_string()
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