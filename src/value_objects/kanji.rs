
use serde::Deserialize;
use super::node_type::NodeType;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Kanji{
    pub name: String,
    pub node_type: NodeType,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}

impl Kanji{
    pub fn create_error_kanji(error_message: &str) -> Kanji{
        Kanji { 
            name: error_message.to_string(),
            node_type: NodeType::Error,
            character: error_message.to_string(), 
            stroke_arrangement: "".to_string(), 
            stroke_count: 0, 
            parent_names: Vec::new() 
        }
    }
}