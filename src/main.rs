use iced::{
    Settings, Application, window::Icon, 
};
pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.icon =  Icon::from_rgba(
        include_bytes!(
            "../resources/images/Kanji Tree R Icon.ico"
        ).to_vec(),
        512,
        512
    ).ok();
    kanji_tree::KanjiTreeApp::run(settings)
}
