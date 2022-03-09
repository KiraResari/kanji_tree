use iced::{Sandbox, Column, Element, Text, Container, Length, Row, Align};

use crate::{kigou_parser::KigouParser, kigou_source::KigouSource, value_objects::Kigou, message::Message, kigou_button::KigouButton, kigou_panel::KigouPanel, reload_button::ReloadButton};

static KANJI_JSON_PATH: &str = "kanji.json";

pub struct KanjiTreeApp{
    kigou_source: KigouSource,
    active_kigou: Kigou,
    child_kigou_buttons: Vec<KigouButton>,
    parent_kigou_buttons: Vec<KigouButton>,
    reload_button: ReloadButton,
}

impl KanjiTreeApp{
    fn load_first_kigou(kanji_source: &KigouSource) -> Kigou{
        let first_kanji_result
             = kanji_source.get_first_element();
        match first_kanji_result{
            Ok(v) => v,
            Err(e) => Kigou::create_error_kigou(&e.to_string())
        }
    }

    fn build_child_kanji_buttons(
        active_kanji: &Kigou,
        kanji_source: &KigouSource
    ) -> Vec<KigouButton>{
        let mut child_kanji_buttons: Vec<KigouButton> =  Vec::new();
        let children 
            = kanji_source.get_children(&active_kanji.name);
        for child in children {
            let kanji_button = KigouButton::new(child);
            child_kanji_buttons.push(kanji_button);
        }
        child_kanji_buttons
    }

    fn build_parent_kanji_buttons(
        active_kanji: &Kigou,
        kanji_source: &KigouSource
    ) -> Vec<KigouButton>{
        let mut parent_kanji_buttons: Vec<KigouButton> =  Vec::new();
        let parents 
            = kanji_source.get_parents(&active_kanji.name);
        for parent in parents {
            let kanji_button = KigouButton::new(parent);
            parent_kanji_buttons.push(kanji_button);
        }
        parent_kanji_buttons
    }

    fn build_main_column(&mut self) -> Column<Message> {

        Column::new()
            .padding(20)
            .push(self.reload_button.view())
            .align_items(Align::Center)
            .push(KanjiTreeApp::build_kanji_button_row(&mut self.parent_kigou_buttons))
            .push(Text::new( "↓".to_string()))
            .push(KigouPanel::from(&self.active_kigou))
            .push(Text::new( "↓".to_string()))
            .push(KanjiTreeApp::build_kanji_button_row(&mut self.child_kigou_buttons))
    }

    fn build_kanji_button_row<'a>(kanji_buttons: &'a mut Vec<KigouButton>) -> Row<'a, Message> {
        let mut children_row: Row<'a, Message> = Row::new().padding(20);
        for kanji_button in kanji_buttons {
            children_row = children_row.push( 
                kanji_button.view()
            );
        }
        children_row
    }

    fn update_kigou_buttons(&mut self){
        self.child_kigou_buttons 
            = KanjiTreeApp::build_child_kanji_buttons(
                &self.active_kigou, 
                &self.kigou_source
            );
        self.parent_kigou_buttons 
            = KanjiTreeApp::build_parent_kanji_buttons(
                &self.active_kigou, 
                &self.kigou_source
            );
    }

    fn reload_kigou_source(&mut self){
        let mut kigou_parser = KigouParser::new();
        self.kigou_source 
            = kigou_parser.parse_kanji_json(
                KANJI_JSON_PATH
            ).unwrap();
        self.reload_active_kigou_or_load_first();
        self.update_kigou_buttons();
    }

    fn reload_active_kigou_or_load_first(&mut self){
        let active_kigou_equivalent 
            = self.kigou_source.get_element(&self.active_kigou.name);
        match active_kigou_equivalent{
            Some(newly_loaded_kigou) => {
                self.active_kigou = newly_loaded_kigou.clone();
            },
            None => {
                self.active_kigou = KanjiTreeApp::load_first_kigou(
                    &self.kigou_source
                );
            }
        }
  }
}

impl Sandbox for KanjiTreeApp {
    type Message = Message;

    fn new() -> KanjiTreeApp {
        let mut kanji_parser = KigouParser::new();
        let kanji_source 
            = kanji_parser.parse_kanji_json(KANJI_JSON_PATH)
                .unwrap();
        let active_kanji
            = KanjiTreeApp::load_first_kigou(&kanji_source);
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
            kigou_source: kanji_source,
            active_kigou: active_kanji,
            child_kigou_buttons: child_kanji_buttons,
            parent_kigou_buttons: parent_kanji_buttons,
            reload_button: ReloadButton::new(),
        }
    }

    fn title(&self) -> String {
        String::from("KanjiTree")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoadKanji(kigou) => {
                self.active_kigou = kigou;
                self.update_kigou_buttons();
            }
            Message::ReloadKigouSource() => {
                self.reload_kigou_source();
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