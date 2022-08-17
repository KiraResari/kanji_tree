use iced::{Element, Text, Container, Image, Length, button::State, Button};

use crate::{value_objects::Kigou, message::Message, fonts};

pub struct KigouDisplayActive{
    kigou: Kigou,
    copy_name_button_state: State
}

impl KigouDisplayActive{

    pub fn new(kigou: &Kigou) -> Self{
        KigouDisplayActive {
            kigou: kigou.clone(),
            copy_name_button_state: State::new() 
        }
    }

    pub fn view(&mut self) -> Element<Message>{
        if self.kigou.uses_image() {
            KigouDisplayActive::build_kigou_image(
                self.kigou.clone()
            ).into()
        }else{
            KigouDisplayActive::build_kigou_character(
                self.kigou.character.clone(),
                &mut self.copy_name_button_state
            ).into()
        }
    }

    fn build_kigou_character(
        character: String, 
        copy_name_button_state: &mut State
    ) -> Button<Message> {
        Button::new(
            copy_name_button_state, 
            Text::new(character.clone())
                .size(64)
                .font(fonts::KANJI)
        ).on_press(Message::CopyStringToClipboard(character))
    }

    fn build_kigou_image<'a>(kigou: Kigou) -> Container<'a, Message> {
        Container::new(
            Image::new(
                format!("resources/images/{}", kigou.image_name)
            )
            .width(Length::Units(64))
        )
    }
}