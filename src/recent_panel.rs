use iced::{Row, Column, Text, Align};

use crate::{kigou_button::KigouButton, message::Message, value_objects::Kigou};

pub struct RecentPanel{
    recent_kigou_button_list: Vec<KigouButton>,
    display_label: bool
}

static MAX_RECENT_KIGOU: usize = 20;

impl RecentPanel{
    pub fn new() -> Self{
        RecentPanel {
            recent_kigou_button_list: Vec::new(),
            display_label: false
        }
    }

    pub fn view(&mut self) -> Column<Message>{
        let mut inner_column = Column::new();
        let mut row = Row::new().padding(1);
        for kigou_button in &mut self.recent_kigou_button_list {
            row = row.push(kigou_button.view());
        }
        inner_column = inner_column.align_items(Align::Center);
        if self.display_label{
            inner_column = inner_column.push(
                Text::new( 
                    "Recent Kigou".to_string()
                ).size(16)
            );
        }
        inner_column = inner_column.push(row);
        Column::new().padding(20).push(inner_column)
    }

    pub fn add(&mut self, kigou: &Kigou){
        let kigou_button = KigouButton::new(kigou.clone());
        self.recent_kigou_button_list.push(kigou_button);
        if self.recent_kigou_button_list.len() > MAX_RECENT_KIGOU {
            self.recent_kigou_button_list.remove(0);
        }
        self.display_label = true;
    }
}