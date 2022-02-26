use std::fmt;

use iced::{container, button};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum NodeType {
    Kanji,
    Radical,
    XPart,
    Dead,
    Error,
}

impl fmt::Display for NodeType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            NodeType::Kanji => write!(f, "Kanji"),
            NodeType::Radical => write!(f, "Radical"),
            NodeType::XPart => write!(f, "XPart"),
            NodeType::Dead => write!(f, "Dead"),
            NodeType::Error => write!(f, "Error"),
        }
    }
}

impl From<NodeType> for Box<dyn container::StyleSheet> {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Kanji => kanji_theme::Container.into(),
            NodeType::Radical => radical_theme::Container.into(),
            NodeType::XPart => kanji_theme::Container.into(),
            NodeType::Dead => kanji_theme::Container.into(),
            NodeType::Error => kanji_theme::Container.into(),
        }
    }
}

impl From<NodeType> for Box<dyn button::StyleSheet> {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Kanji => kanji_theme::Button.into(),
            NodeType::Radical => radical_theme::Button.into(),
            NodeType::XPart => kanji_theme::Button.into(),
            NodeType::Dead => kanji_theme::Button.into(),
            NodeType::Error => kanji_theme::Button.into(),
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