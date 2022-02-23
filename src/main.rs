use iced::{
    Settings, Application, 
};
pub fn main() -> iced::Result {
    kanji_tree::KanjiTreeApp::run(Settings::default())
}
