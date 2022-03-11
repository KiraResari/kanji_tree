use iced::{Sandbox, Column, Element, Text, Container, Length, Row, Align, Font};

use crate::{kigou_parser::KigouParser, kigou_source::KigouSource, value_objects::Kigou, message::Message, kigou_button::KigouButton, kigou_panel::KigouPanel, reload_button::ReloadButton, search_panel::SearchPanel};

static KANJI_JSON_PATH: &str = "resources/kanji.json";

pub struct KanjiTreeApp{
    kigou_source: KigouSource,
    active_kigou: Kigou,
    child_kigou_buttons: Vec<KigouButton>,
    parent_kigou_buttons: Vec<KigouButton>,
    reload_button: ReloadButton,
    search_panel: SearchPanel,
    display_message: String,
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
            .push(self.build_display_message())
            .push(self.reload_button.view())
            .push(self.search_panel.view())
            .align_items(Align::Center)
            .push(KanjiTreeApp::build_kanji_button_row(
                &mut self.parent_kigou_buttons)
            )
            .push(KanjiTreeApp::build_arrow_if_necessary(
                self.active_kigou.has_parents())
            )
            .push(KigouPanel::from(&self.active_kigou))
            .push(KanjiTreeApp::build_arrow_if_necessary(
                self.kigou_source.has_children(
                    &self.active_kigou.name
                )
            )
            )
            .push(KanjiTreeApp::build_kanji_button_row(&mut self.child_kigou_buttons))
    }

    fn build_display_message(&self) -> Text {
        Text::new(&self.display_message)
            .font(
                Font::External{
                    name: "msgothic",
                    bytes: include_bytes!("../fonts/msgothic.ttc")
                }
            )
    }

    fn build_arrow_if_necessary(arrow_should_be_displayed: bool) -> Text {
        if arrow_should_be_displayed {
            return Text::new( "↓".to_string())
                .size(32)
        }
        Text::new( "".to_string())
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

    fn load_kigou(&mut self, kigou: Kigou){
        self.active_kigou = kigou;
        self.update_kigou_buttons_and_message();
    }

    fn update_kigou_buttons_and_message(&mut self){
        self.update_kigou_buttons();
        self.display_message = format!(
            "Loaded {}",
            self.active_kigou.to_string()
        );
    }

    fn update_kigou_buttons(&mut self) {
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
        self.display_message = "Reloading kanji.json...".to_string();
        let mut kigou_parser = KigouParser::new();
        self.kigou_source 
            = kigou_parser.parse_kanji_json(
                KANJI_JSON_PATH
            ).unwrap();
        self.reload_active_kigou_or_load_first();
        self.update_kigou_buttons();
        self.display_message = "Finished reloading kanji.json!".to_string();
    }

    fn reload_active_kigou_or_load_first(&mut self){
        let active_kigou_equivalent 
            = self.kigou_source.get_kigou_by_name(
                &self.active_kigou.name
            );
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

    fn search_for_kigou(&mut self, query: String){
        let character_search_result
            = self.kigou_source.get_kigou_by_character(&query);
        match character_search_result{
            Some(matched_kigou ) =>{
                self.active_kigou = matched_kigou.clone();
                self.update_kigou_buttons_and_message();
            }
            None => {
                self.search_for_kigou_by_exact_name(query);
            }
        }
    }

    fn search_for_kigou_by_exact_name(&mut self, query: String){
        let character_search_result
            = self.kigou_source.get_kigou_by_name(&query);
        match character_search_result{
            Some(matched_kigou ) =>{
                self.active_kigou = matched_kigou.clone();
                self.update_kigou_buttons_and_message();
            }
            None => {
                self.search_for_kigou_by_fuzzy_name(query);
            }
        }
    }

    fn search_for_kigou_by_fuzzy_name(&mut self, query: String){
        let character_search_result
            = self.kigou_source.get_kigou_by_name_fuzzy(&query);
        match character_search_result{
            Some(matched_kigou ) =>{
                self.active_kigou = matched_kigou.clone();
                self.update_kigou_buttons_and_message();
            }
            None => {
                self.display_message = format!(
                    "Could not find a Kigou for search string: {}",
                    query
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
            search_panel: SearchPanel::new(),
            display_message: "Welcome to the Kanji Tree!".to_string()
        }
    }

    fn title(&self) -> String {
        String::from("KanjiTree")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoadKigou(kigou) => {
                self.load_kigou(kigou);
            }
            Message::ReloadKigouSource() => {
                self.reload_kigou_source();
            }
            Message::SearchForKigou(query) => {
                self.search_for_kigou(query);
            }
            Message::SearchBoxInputChanged(input) => {
                self.search_panel.update(input);
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