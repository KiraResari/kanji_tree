
use serde::Deserialize;
use super::node_type::NodeType;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ParsedKanjiJsonElement{
    pub name: String,
    pub node_type: NodeType,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}