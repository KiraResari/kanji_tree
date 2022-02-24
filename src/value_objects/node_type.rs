use std::fmt;

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