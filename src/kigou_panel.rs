use iced::{Text, Column, Align, Row, Container};

use crate::{value_objects::Kigou, message::Message, kigou_display_builder::KigouDisplayBuilder};

pub struct KigouPanel{
}

impl KigouPanel{
    pub fn from(kigou: &Kigou) -> Container<Message>{

        Container::new(
            Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new(kigou.stroke_count.to_string()))
                .push(KigouPanel::build_kigou_column(kigou))
                .push(Text::new(kigou.stroke_arrangement.to_string()))
        ).style(kigou.clone().kigou_type)
    }

    fn build_kigou_column(kigou: &Kigou) -> Column<Message>{
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(kigou.name.to_string()))
            .push(KigouDisplayBuilder::build_kigou_display(kigou, 64))
            .push(Text::new(kigou.kigou_type.to_string()))
    }
}