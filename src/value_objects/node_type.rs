use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum NodeType {
    Kanji,
    Radical,
    XPart,
    Dead,
    Error,
}