use iced::{Button, Text, Row, TextInput, text_input, button};

use crate::{message::Message, fonts, value_objects::KigouType};

pub struct SearchPanel{
    search_button_state: button::State,
    kanji_search_button_state: button::State,
    radical_search_button_state: button::State,
    x_part_search_button_state: button::State,
    kana_search_button_state: button::State,
    dead_search_button_state: button::State,
    search_box_state: text_input::State,
    search_string: String
}

impl SearchPanel{
    pub fn new() -> Self{
        SearchPanel {
            search_button_state: button::State::new(),
            kanji_search_button_state: button::State::new(),
            radical_search_button_state: button::State::new(),
            x_part_search_button_state: button::State::new(),
            kana_search_button_state: button::State::new(),
            dead_search_button_state: button::State::new(),
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
            .font(fonts::KANJI)
            .padding(15)
            .size(32)
            .on_submit(Message::SearchForKigou(self.search_string.clone(), None))
        )
        .push(
            Button::new(
                &mut self.search_button_state, 
                Text::new("🔍")
                    .font(fonts::SYMBOL)
                    .size(52)
            )
            .on_press(
                Message::SearchForKigou(
                    self.search_string.clone(),
                    None
                )
            )
        )
        .push(
            Button::new(
                &mut self.kanji_search_button_state, 
                Text::new("漢")
                    .font(fonts::KANJI)
                    .size(52)
            )
            .on_press(
                Message::SearchForKigou(
                    self.search_string.clone(),
                    Some(KigouType::Kanji)
                )
            ).style(KigouType::Kanji)
        )
        .push(
            Button::new(
                &mut self.radical_search_button_state, 
                Text::new("氵")
                    .font(fonts::KANJI)
                    .size(52)
            )
            .on_press(
                Message::SearchForKigou(
                    self.search_string.clone(),
                    Some(KigouType::Radical)
                )
            ).style(KigouType::Radical)
        )
        .push(
            Button::new(
                &mut self.x_part_search_button_state, 
                Text::new("X")
                    .font(fonts::SYMBOL)
                    .size(52)
            )
            .on_press(
                Message::SearchForKigou(
                    self.search_string.clone(),
                    Some(KigouType::XPart)
                )
            ).style(KigouType::XPart)
        )
        .push(
            Button::new(
                &mut self.kana_search_button_state, 
                Text::new("オ")
                    .font(fonts::KANJI)
                    .size(52)
            )
            .on_press(
                Message::SearchForKigou(
                    self.search_string.clone(),
                    Some(KigouType::Kana)
                )
            ).style(KigouType::Kana)
        )
        .push(
            Button::new(
                &mut self.dead_search_button_state, 
                Text::new("D")
                    .font(fonts::SYMBOL)
                    .size(52)
            )
            .on_press(
                Message::SearchForKigou(
                    self.search_string.clone(),
                    Some(KigouType::Dead)
                )
            ).style(KigouType::Dead)
        )
    }

    pub fn update(&mut self, input: String){
        self.search_string = input;
    }
}