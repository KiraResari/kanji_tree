use iced::{
    Settings, Application, window::Icon, 
};

static ICON: &[u8]
     = include_bytes!("../resources/images/Kanji Tree R Icon.ico");
const ICON_HEIGHT: u32 = 256;
const ICON_WIDTH: u32 = 256;

pub fn main() -> iced::Result {
    let settings = build_settings();
    kanji_tree::KanjiTreeApp::run(settings)
}

fn build_settings() -> Settings<()> {
    let image
        = image::load_from_memory(ICON).unwrap();
    let icon = Icon::from_rgba(
        image.as_bytes().to_vec(),
        ICON_HEIGHT,
        ICON_WIDTH
    ).unwrap();
    Settings {
        window: iced::window::Settings {
            icon: Some(icon), 
            ..Default::default()
        },
        ..Default::default()
    }
}
