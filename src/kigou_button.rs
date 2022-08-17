use iced::{button::State, Button};

use crate::{value_objects::Kigou, message::Message, kigou_display_buttons::KigouDisplayButtons};

pub struct KigouButton{
    kigou: Kigou,
    state: State,
    kigou_display: KigouDisplayButtons
}

impl KigouButton{
    pub fn new(kigou: Kigou) -> Self{
        KigouButton { 
            kigou: kigou.clone(), 
            state: State::new(),
            kigou_display: KigouDisplayButtons::new(&kigou)
        }
    }

    pub fn view(&mut self) -> Button<Message> {
        Button::new(
            &mut self.state, 
            self.kigou_display.view()
        ).on_press(Message::LoadKigou(self.kigou.clone()))
        .style(self.kigou.clone().kigou_type)
    }
}