
use serde::Deserialize;
use super::{kigou_type::KigouType, Kigou};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Radical{
    pub name: String,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}

impl From<&Radical> for Kigou{
    fn from(radical_node: &Radical) -> Self{
        let clone = radical_node.clone();
        Self{
            name: clone.name,
            character: clone.character,
            stroke_arrangement: clone.stroke_arrangement,
            stroke_count: clone.stroke_count,
            parent_names: clone.parent_names,
            kigou_type: KigouType::Radical
        }
    }
}