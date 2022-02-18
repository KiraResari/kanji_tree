use iced::{Sandbox, Column, Element, Text, Font, Container, Length};

pub struct KanjiTreeApp{
    active_kanji: String,
}

impl Default for KanjiTreeApp {
    fn default() -> Self {
        Self {
            active_kanji: String::from("狐"),
        }
    }
}

impl Sandbox for KanjiTreeApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("KanjiTree")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                
            }
            Message::DecrementPressed => {
                
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .push(
                Text::new(
                    self.active_kanji.to_string()
                ).size(64)
                .font(Font::External{
                    name: "msgothic",
                    bytes: include_bytes!("../fonts/msgothic.ttc")
                })
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}