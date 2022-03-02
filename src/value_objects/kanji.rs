
use serde::Deserialize;
use super::{kigou_type::KigouType, Kigou};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Kanji{
    pub name: String,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}

impl From<&Kanji> for Kigou{
    fn from(kanji_node: &Kanji) -> Self{
        let clone = kanji_node.clone();
        Self{
            name: clone.name,
            character: clone.character,
            stroke_arrangement: clone.stroke_arrangement,
            stroke_count: clone.stroke_count,
            parent_names: clone.parent_names,
            kigou_type: KigouType::Kanji,
            image_name: "".to_string(),
        }
    }
}