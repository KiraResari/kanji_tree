use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum NodeType {
    Kanji,
    Radical,
    XPart,
    Dead,
}