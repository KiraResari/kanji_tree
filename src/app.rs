use iced::{Sandbox, Column, Element, Button, Text};

pub struct KanjiTreeApp{
    activeKanji: String,
}

impl Default for KanjiTreeApp {
    fn default() -> Self {
        Self {
            activeKanji: String::from("狐"),
        }
    }
}

impl Sandbox for KanjiTreeApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
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
        Column::new()
            .padding(20)

            .push(Text::new(self.activeKanji.to_string()).size(50))

            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}