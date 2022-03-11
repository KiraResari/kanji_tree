use iced::{
    Settings, Application, window::Icon, 
};
pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.icon =  Icon::from_rgba(
        include_bytes!(
            "../resources/images/Kanji Tree R Icon 32.bmp"
        ).to_vec(),
        32,
        32
    ).ok();
    kanji_tree::KanjiTreeApp::run(settings)
}
