use iced::{Text, Column, Align, Row, Container, button::State, Button};

use crate::{value_objects::Kigou, message::Message, fonts, kigou_display_builder::KigouDisplayBuilder};

pub struct KigouPanel{
    kigou: Kigou,
    copy_name_button_state: State
}

impl KigouPanel{
    pub fn new(kigou: &Kigou) -> Self{
        KigouPanel {
            kigou: kigou.clone(),
            copy_name_button_state: State::new() 
        }
    }

    pub fn view(&mut self) -> Container<Message>{

        Container::new(
            Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new(self.kigou.stroke_count.to_string()))
                .push(
                    Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(
                            &mut self.copy_name_button_state, 
                            Text::new(self.kigou.clone().name)
                            .font(fonts::SYMBOL)
                        ).on_press(Message::CopyStringToClipboard(self.kigou.clone().name))
                    )
                    .push(
                        KigouDisplayBuilder::build_kigou_display(
                            &self.kigou, 64
                        )
                    )
                    .push(Text::new(self.kigou.kigou_type.to_string()))
                )
                .push(
                    Text::new(self.kigou.clone().stroke_arrangement.to_string())
                    .font(fonts::KANJI)
                    .size(48)
                )
        ).style(self.kigou.clone().kigou_type)
    }

}