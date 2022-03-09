use iced::{Button, Text, Row, TextInput, text_input, button};

use crate::{message::Message};

pub struct SearchPanel{
    button_state: button::State,
    search_box_state: text_input::State,
    search_string: String
}

impl SearchPanel{
    pub fn new() -> Self{
        SearchPanel {
            button_state: button::State::new(),
            search_box_state: text_input::State::new(),
            search_string: "".to_string(),
        }
    }

    pub fn view(&mut self) -> Row<Message> {
        Row::new()
        .push(
            TextInput::new(
                &mut self.search_box_state,
                "Search by character or name",
                &self.search_string,
                Message::SearchBoxInputChanged,
            )
            .on_submit(Message::SearchForKigou(self.search_string.clone()))
        )
        .push(
            Button::new(
                &mut self.button_state, 
                Text::new("🔍")
            ).on_press(Message::SearchForKigou(self.search_string.clone()))
        )
    }

    pub fn update(&mut self, input: String){
        self.search_string = input;
    }


}