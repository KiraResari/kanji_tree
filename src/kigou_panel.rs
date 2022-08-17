use iced::{Text, Column, Align, Row, Container, button::State, Button};

use crate::{value_objects::Kigou, message::Message, fonts, kigou_display_active::KigouDisplayActive};

pub struct KigouPanel{
    kigou: Kigou,
    copy_name_button_state: State,
    kigou_display: KigouDisplayActive
}

impl KigouPanel{
    pub fn new(kigou: &Kigou) -> Self{
        KigouPanel {
            kigou: kigou.clone(),
            copy_name_button_state: State::new() ,
            kigou_display: KigouDisplayActive::new(&kigou)
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
                    .push( self.kigou_display.view())
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