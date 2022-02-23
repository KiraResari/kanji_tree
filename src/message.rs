use crate::value_objects::Kanji;

#[derive(Debug, Clone)]
pub enum Message {
    LoadKanji(Kanji),
}