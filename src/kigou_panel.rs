use iced::{Text, Column, Align, Row, Container, button::State};

use crate::{value_objects::Kigou, message::Message, fonts, kigou_display_builder::KigouDisplayBuilder};

pub struct KigouPanel{
    kigou: Kigou,
    _copy_name_button_state: State
}

impl KigouPanel{
    pub fn new(kigou: &Kigou) -> Self{
        KigouPanel {
            kigou: kigou.clone(),
            _copy_name_button_state: State::new() 
        }
    }

    pub fn view(&mut self) -> Container<Message>{

        Container::new(
            Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new(self.kigou.stroke_count.to_string()))
                .push(self.build_kigou_column())
                .push(
                    Text::new(self.kigou.clone().stroke_arrangement.to_string())
                    .font(fonts::KANJI)
                    .size(48)
                )
                    
        ).style(self.kigou.clone().kigou_type)
    }

    fn build_kigou_column(&self) -> Column<Message>{
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Text::new(self.kigou.name.to_string())
            )
            .push(KigouDisplayBuilder::build_kigou_display(&self.kigou, 64))
            .push(Text::new(self.kigou.kigou_type.to_string()))
    }
}