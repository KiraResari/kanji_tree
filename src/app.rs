use iced::{Sandbox, Column, Element, Text, Container, Length, Row, Align, Color};
use arboard::Clipboard;

use crate::{
    kigou_parser::KigouParser,
    kigou_source::KigouSource,
    value_objects::Kigou,
    message::Message,
    kigou_button::KigouButton,
    kigou_panel::KigouPanel,
    reload_button::ReloadButton,
    search_panel::SearchPanel,
    copy_button::CopyButton,
    fonts
};

static KANJI_JSON_PATH: &str = "resources/kanji.json";
static KIGOU_PER_ROW: usize = 20;

pub struct KanjiTreeApp{
    kigou_source: KigouSource,
    active_kigou: Kigou,
    child_kigou_buttons: Vec<KigouButton>,
    parent_kigou_buttons: Vec<KigouButton>,
    reload_button: ReloadButton,
    search_panel: SearchPanel,
    display_message: String,
    copy_button: CopyButton,
    clipboard: Clipboard,
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

    fn build_child_kigou_buttons(
        active_kigou: &Kigou,
        kigou_source: &KigouSource
    ) -> Vec<KigouButton>{
        let mut child_kigou_buttons: Vec<KigouButton> =  Vec::new();
        let children 
            = kigou_source.get_children(&active_kigou.name);
        for child in children {
            let kanji_button = KigouButton::new(child);
            child_kigou_buttons.push(kanji_button);
        }
        child_kigou_buttons
    }

    fn build_parent_kigou_buttons(
        active_kigou: &Kigou,
        kigou_source: &KigouSource
    ) -> Vec<KigouButton>{
        let mut parent_kigou_buttons: Vec<KigouButton> =  Vec::new();
        let parents 
            = kigou_source.get_parents(&active_kigou.name);
        for parent in parents {
            let kanji_button = KigouButton::new(parent);
            parent_kigou_buttons.push(kanji_button);
        }
        parent_kigou_buttons
    }

    fn build_main_column(&mut self) -> Column<Message> {

        Column::new()
            .padding(20)
            .push(self.build_display_message())
            .push(self.reload_button.view())
            .push(self.search_panel.view())
            .align_items(Align::Center)
            .push(KanjiTreeApp::build_kigou_button_block(
                &mut self.parent_kigou_buttons)
            )
            .push(KanjiTreeApp::build_arrow_if_necessary(
                self.active_kigou.has_parents())
            )
            .push(KanjiTreeApp::build_kigou_panel_row(&self.active_kigou, &mut self.copy_button))
            .push(KanjiTreeApp::build_arrow_if_necessary(
                self.kigou_source.has_children(
                    &self.active_kigou.name
                )
            )
            )
            .push(KanjiTreeApp::build_kigou_button_block(&mut self.child_kigou_buttons))
            .push(KanjiTreeApp::build_version_row())
    }

    fn build_display_message(&self) -> Text {
        Text::new(&self.display_message)
            .font(fonts::KANJI)
    }

    fn build_arrow_if_necessary(arrow_should_be_displayed: bool) -> Text {
        if arrow_should_be_displayed {
            return Text::new( "↓".to_string())
                .size(32)
        }
        Text::new( "".to_string())
    }

    fn build_kigou_button_block<'a>(kigou_buttons: &'a mut Vec<KigouButton>) -> Column<'a, Message> {
        let mut kigou_button_column: Column<'a, Message> = Column::new().align_items(Align::Center);
        for chunk in kigou_buttons.chunks_mut(KIGOU_PER_ROW) {
            let mut row = Row::new().padding(1);
            for kigou_button in chunk {
              row = row.push(kigou_button.view());
            }
            kigou_button_column = kigou_button_column.push(row);
          }
        kigou_button_column
    }

    fn build_kigou_panel_row<'a>(active_kigou: &'a Kigou, copy_button: &'a mut CopyButton) -> Row<'a, Message> {
        let kigou_button_row: Row<'a, Message> = Row::new()
            .push(KigouPanel::from(active_kigou))
            .push(copy_button.view())
            .align_items(Align::Center);
        kigou_button_row
    }

    fn build_version_row<'a>() -> Row<'a, Message>{
        let version_button_row: Row<'a, Message> = Row::new()
            .push(
                Text::new( format!("Kanji Tree R version {}", env!("CARGO_PKG_VERSION")))
                    .size(16).color(Color::from_rgb8(127, 127, 127))
            );
        version_button_row
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
            = KanjiTreeApp::build_child_kigou_buttons(
                &self.active_kigou, 
                &self.kigou_source
            );
        self.parent_kigou_buttons 
            = KanjiTreeApp::build_parent_kigou_buttons(
                &self.active_kigou, 
                &self.kigou_source
            );
    }

    fn reload_kigou_source(&mut self){
        self.display_message = "Reloading kanji.json...".to_string();
        let mut kigou_parser = KigouParser::new();


        let kigou_parse_result = kigou_parser.parse_kanji_json(
            KANJI_JSON_PATH
        );
        match kigou_parse_result{
            Ok(v) => {
                self.kigou_source = v; 
                self.reload_active_kigou_or_load_first();
                self.update_kigou_buttons();
                self.display_message = "Finished reloading kanji.json!".to_string();
            },
            Err(e) => {
                self.display_message = format!("Failed to reload kanji.json because of error: {}", e.to_string());
            }
        }

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

    fn copy_active_kigou_name(&mut self){
        let result = self.clipboard.set_text(self.active_kigou.name.clone());
        match result{
            Ok(_) => {
                self.display_message = "Copied Kigou name to clipboard".to_string();
            }
            Err(e) => {
                self.display_message = format!("Failed to copy Kigou to clipboard because of error: {}", e.to_string());
            }
        }
    }

}

impl Sandbox for KanjiTreeApp {
    type Message = Message;

    fn new() -> KanjiTreeApp {
        let mut kigou_parser = KigouParser::new();
        let kigou_parse_result 
            = kigou_parser.parse_kanji_json(KANJI_JSON_PATH);

        let kigou_source;
        let display_message;
        match kigou_parse_result{
            Ok(v) => {
                kigou_source = v;
                display_message = "Welcome to the Kanji Tree!".to_string();
            },
            Err(e) => {
                display_message = format!("Error loading kanji.json: {}", e.to_string());
                kigou_source = KigouSource::create_kigou_source_for_invalid_json(e);
            }
        }

        let active_kanji
            = KanjiTreeApp::load_first_kigou(&kigou_source);
        let child_kanji_buttons 
            = KanjiTreeApp::build_child_kigou_buttons(
                &active_kanji,
                &kigou_source
            );
        let parent_kanji_buttons 
            = KanjiTreeApp::build_parent_kigou_buttons(
                &active_kanji,
                &kigou_source
            );
        KanjiTreeApp{
            kigou_source,
            active_kigou: active_kanji,
            child_kigou_buttons: child_kanji_buttons,
            parent_kigou_buttons: parent_kanji_buttons,
            reload_button: ReloadButton::new(),
            search_panel: SearchPanel::new(),
            display_message,
            copy_button: CopyButton::new(),
            clipboard: Clipboard::new().unwrap()
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
            Message::CopyActiveKigouName() => {
                self.copy_active_kigou_name();
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