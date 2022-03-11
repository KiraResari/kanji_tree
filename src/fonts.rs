use iced::Font;

pub struct Fonts{

}

impl Fonts{
    pub fn ms_gothic() -> Font {
        Font::External{
            name: "msgothic",
            bytes: include_bytes!("../resources/fonts/msgothic.ttc")
        }
    }

    pub fn segoe_ui_symbol() -> Font {
        Font::External{
            name: "seguisym",
            bytes: include_bytes!("../resources/fonts/seguisym.ttf")
        }
    }
}