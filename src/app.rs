use iced::{Sandbox, Column, Element, Text, Font, Container, Length, Row, Align};

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

    fn build_main_column(&mut self) -> Column<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(self.build_active_kanji_text())
            .push(Text::new( "↓".to_string()))
            .push(self.build_children_row())
    }

    fn build_active_kanji_text(&mut self) -> Text {
        Text::new(
            self.active_kanji.character.to_string()
        ).size(64)
        .font(Font::External{
            name: "msgothic",
            bytes: include_bytes!("../fonts/msgothic.ttc")
        })
    }

    fn build_children_row(&mut self) -> Row<Message> {
        let children 
            = self.kanji_source.get_children(&self.active_kanji.name);
        let mut children_row: Row<Message> = Row::new().padding(20);
        for child in children {
            children_row = children_row.push(                
                self.build_child_text(child)
            );
        }
        children_row
    }

    fn build_child_text(&mut self, child: Kanji) -> Text {
        Text::new(
            child.character.to_string()
        ).size(32)
        .font(Font::External{
            name: "msgothic",
            bytes: include_bytes!("../fonts/msgothic.ttc")
        })
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
        let main_column = self.build_main_column();

        Container::new(main_column)
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