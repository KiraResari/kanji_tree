use std::fmt;

use iced::{container, button};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum KigouType {
    Kanji,
    Radical,
    XPart,
    Dead,
    Error,
}

impl fmt::Display for KigouType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            KigouType::Kanji => write!(f, "Kanji"),
            KigouType::Radical => write!(f, "Radical"),
            KigouType::XPart => write!(f, "XPart"),
            KigouType::Dead => write!(f, "Dead"),
            KigouType::Error => write!(f, "Error"),
        }
    }
}

impl From<KigouType> for Box<dyn container::StyleSheet> {
    fn from(node_type: KigouType) -> Self {
        match node_type {
            KigouType::Kanji => kanji_theme::Container.into(),
            KigouType::Radical => radical_theme::Container.into(),
            KigouType::XPart => kanji_theme::Container.into(),
            KigouType::Dead => kanji_theme::Container.into(),
            KigouType::Error => kanji_theme::Container.into(),
        }
    }
}

impl From<KigouType> for Box<dyn button::StyleSheet> {
    fn from(node_type: KigouType) -> Self {
        match node_type {
            KigouType::Kanji => kanji_theme::Button.into(),
            KigouType::Radical => radical_theme::Button.into(),
            KigouType::XPart => kanji_theme::Button.into(),
            KigouType::Dead => kanji_theme::Button.into(),
            KigouType::Error => kanji_theme::Button.into(),
        }
    }
}

mod kanji_theme {
    use iced::{container, Color, button};

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::BLACK),
                background: Color::from_rgb8(255, 127, 0).into(),
                border_color: Color::BLACK,
                border_width: 2.0,
                border_radius: 8.0,
            }
        }
    }
    pub struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Color::from_rgb8(255, 127, 0).into(),
                text_color: Color::BLACK,
                border_color: Color::BLACK,
                border_width: 2.0,
                border_radius: 8.0,
                ..button::Style::default()
            }
        }
        fn hovered(&self) -> button::Style {
            button::Style {
                background: Color::from_rgb8(255, 192, 127).into(),
                text_color: Color::BLACK,
                border_color: Color::BLACK,
                border_width: 2.0,
                border_radius: 8.0,
                ..button::Style::default()
            }
        }
    }
}

mod radical_theme {
    use iced::{container, Color, button};

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::BLACK),
                background: Color::from_rgb8(0, 255, 127).into(),
                border_color: Color::BLACK,
                border_width: 2.0,
                border_radius: 8.0,
            }
        }
    }
    pub struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Color::from_rgb8(0, 255, 127).into(),
                text_color: Color::BLACK,
                border_color: Color::BLACK,
                border_width: 2.0,
                border_radius: 8.0,
                ..button::Style::default()
            }
        }
        fn hovered(&self) -> button::Style {
            button::Style {
                background: Color::from_rgb8(127, 255, 192).into(),
                text_color: Color::BLACK,
                border_color: Color::BLACK,
                border_width: 2.0,
                border_radius: 8.0,
                ..button::Style::default()
            }
        }
    }
}