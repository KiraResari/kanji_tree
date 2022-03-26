use iced::Font;
use array_concat::*;

const MS_GOTHIC_PART_1: [u8; 9176636] = *include_bytes!("../resources/fonts/msgothic.ttc");
const MS_GOTHIC_PART_2: [u8; 9165480] = *include_bytes!("../resources/fonts/msgothic_0.ttc");
const MS_GOTHIC: [u8; concat_arrays_size!(MS_GOTHIC_PART_1, MS_GOTHIC_PART_2)]
 = concat_arrays!(MS_GOTHIC_PART_1, MS_GOTHIC_PART_2);

pub const KANJI: Font = Font::External{
    name: "simsun",
    bytes: include_bytes!("../resources/fonts/Noto Sans Mono CJK JP Regular.otf")
};

pub struct Fonts{

}

impl Fonts{
    pub fn ms_gothic() -> Font {
        
        Font::External{
            name: "msgothic",
            bytes: &MS_GOTHIC
        }
    }

    pub fn segoe_ui_symbol() -> Font {
        Font::External{
            name: "seguisym",
            bytes: include_bytes!("../resources/fonts/seguisym.ttf")
        }
    }
}