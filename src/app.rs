use iced::{Sandbox, Column, Element, Text, Container, Length, Row, Align};

use crate::{kanji_parser::KanjiParser, kanji_source::KanjiSource, value_objects::Kanji, message::Message, kanji_button::KanjiButton, kanji_panel::KanjiPanel};

static KANJI_JSON_PATH: &str = "kanji.json";

pub struct KanjiTreeApp{
    kanji_source: KanjiSource,
    active_kanji: Kanji,
    child_kanji_buttons: Vec<KanjiButton>,
    parent_kanji_buttons: Vec<KanjiButton>,
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

    fn build_child_kanji_buttons(
        active_kanji: &Kanji,
        kanji_source: &KanjiSource
    ) -> Vec<KanjiButton>{
        let mut child_kanji_buttons: Vec<KanjiButton> =  Vec::new();
        let children 
            = kanji_source.get_children(&active_kanji.name);
        for child in children {
            let kanji_button = KanjiButton::new(child);
            child_kanji_buttons.push(kanji_button);
        }
        child_kanji_buttons
    }

    fn build_parent_kanji_buttons(
        active_kanji: &Kanji,
        kanji_source: &KanjiSource
    ) -> Vec<KanjiButton>{
        let mut parent_kanji_buttons: Vec<KanjiButton> =  Vec::new();
        let parents 
            = kanji_source.get_parents(&active_kanji.name);
        for parent in parents {
            let kanji_button = KanjiButton::new(parent);
            parent_kanji_buttons.push(kanji_button);
        }
        parent_kanji_buttons
    }

    fn build_main_column(&mut self) -> Column<Message> {

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(KanjiTreeApp::build_kanji_button_row(&mut self.parent_kanji_buttons))
            .push(Text::new( "↓".to_string()))
            .push(KanjiPanel::from(self.active_kanji.clone()))
            .push(Text::new( "↓".to_string()))
            .push(KanjiTreeApp::build_kanji_button_row(&mut self.child_kanji_buttons))
    }

    fn build_kanji_button_row<'a>(kanji_buttons: &'a mut Vec<KanjiButton>) -> Row<'a, Message> {
        let mut children_row: Row<'a, Message> = Row::new().padding(20);
        for kanji_button in kanji_buttons {
            children_row = children_row.push( 
                kanji_button.view()
            );
        }
        children_row
    }

    fn update_kanji_buttons(&mut self){
        self.child_kanji_buttons 
            = KanjiTreeApp::build_child_kanji_buttons(
                &self.active_kanji, 
                &self.kanji_source
            );
        self.parent_kanji_buttons 
            = KanjiTreeApp::build_parent_kanji_buttons(
                &self.active_kanji, 
                &self.kanji_source
            );
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
        let child_kanji_buttons 
            = KanjiTreeApp::build_child_kanji_buttons(
                &active_kanji,
                &kanji_source
            );
        let parent_kanji_buttons 
            = KanjiTreeApp::build_parent_kanji_buttons(
                &active_kanji,
                &kanji_source
            );
        KanjiTreeApp{
            kanji_source,
            active_kanji,
            child_kanji_buttons,
            parent_kanji_buttons,
        }
    }

    fn title(&self) -> String {
        String::from("KanjiTree")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoadKanji(kanji) => {
                self.active_kanji = kanji;
                self.update_kanji_buttons();
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