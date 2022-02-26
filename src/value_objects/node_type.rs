use std::fmt;

use iced::container;
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

mod kanji_theme {
    use iced::{container, Color};

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
}

mod radical_theme {
    use iced::{container, Color};

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
}