
use serde::Deserialize;
use super::{kigou_type::KigouType, Kigou};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct XPart{
    pub name: String,
    pub image_name: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}

impl From<&XPart> for Kigou{
    fn from(kanji_node: &XPart) -> Self{
        let clone = kanji_node.clone();
        Self{
            name: clone.name,
            character: "".to_string(),
            stroke_arrangement: clone.stroke_arrangement,
            stroke_count: clone.stroke_count,
            parent_names: clone.parent_names,
            kigou_type: KigouType::XPart,
            image_name: clone.image_name,
        }
    }
}