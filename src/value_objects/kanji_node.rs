
use serde::Deserialize;
use super::{node_type::NodeType, Kanji};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct KanjiNode{
    pub name: String,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}

impl From<&KanjiNode> for Kanji{
    fn from(kanji_node: &KanjiNode) -> Self{
        let clone = kanji_node.clone();
        Self{
            name: clone.name,
            character: clone.character,
            stroke_arrangement: clone.stroke_arrangement,
            stroke_count: clone.stroke_count,
            parent_names: clone.parent_names,
            node_type: NodeType::Kanji
        }
    }
}