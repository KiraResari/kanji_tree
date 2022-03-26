use iced::Font;

pub const KANJI: Font = Font::External{
    name: "simsun",
    bytes: include_bytes!("../resources/fonts/Noto Sans Mono CJK JP Regular.otf")
};

pub const SYMBOL: Font = Font::External{
    name: "seguisym",
    bytes: include_bytes!("../resources/fonts/seguisym.ttf")
};