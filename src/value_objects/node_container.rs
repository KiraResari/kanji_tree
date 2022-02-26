use serde::Deserialize;

use crate::kanji_source::KanjiSource;

use super::{KanjiNode, RadicalNode, Kanji};

#[derive(Deserialize)]
pub struct NodeContainer{
    #[serde(default)]
    pub kanji: Vec<KanjiNode>,
    #[serde(default)]
    pub radical: Vec<RadicalNode>,
}

impl Into<KanjiSource> for NodeContainer{
    fn into(self) -> KanjiSource{
        let mut kanji: Vec<Kanji> = Vec::new();
        kanji.extend(
           self.kanji.iter().map(|v| v.into())
        );
        kanji.extend(
            self.radical.iter().map(|v| v.into())
         );
        KanjiSource{kanji}
    }
}