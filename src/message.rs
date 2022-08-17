use crate::value_objects::{Kigou, KigouType};

#[derive(Debug, Clone)]
pub enum Message {
    LoadKigou(Kigou),
    ReloadKigouSource(),
    SearchForKigou(String, Option<KigouType>),
    SearchBoxInputChanged(String),
    CopyActiveKigouName(),
}