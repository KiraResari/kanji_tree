
use serde::Deserialize;
use super::{kigou_type::KigouType, Kigou};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct DeadKanji{
    pub name: String,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}

impl From<&DeadKanji> for Kigou{
    fn from(kanji_node: &DeadKanji) -> Self{
        let clone = kanji_node.clone();
        Self{
            name: clone.name,
            character: clone.character,
            stroke_arrangement: clone.stroke_arrangement,
            stroke_count: clone.stroke_count,
            parent_names: clone.parent_names,
            kigou_type: KigouType::Dead,
            image_name: "".to_string(),
        }
    }
}