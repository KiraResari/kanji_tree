use crate::value_objects::Kigou;

#[derive(Debug, Clone)]
pub enum Message {
    LoadKanji(Kigou),
    ReloadKigouSource(),
    SearchForKigou(String),
    SearchBoxInputChanged(String),
}