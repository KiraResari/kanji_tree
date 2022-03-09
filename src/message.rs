use crate::value_objects::Kigou;

#[derive(Debug, Clone)]
pub enum Message {
    LoadKigou(Kigou),
    ReloadKigouSource(),
    SearchForKigou(String),
    SearchBoxInputChanged(String),
}