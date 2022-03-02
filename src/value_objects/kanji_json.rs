use serde::Deserialize;

use crate::kigou_source::KigouSource;

use super::{Kanji, Radical, Kigou, XPart};

#[derive(Deserialize)]
pub struct KanjiJson{
    #[serde(default)]
    pub kanji: Vec<Kanji>,
    #[serde(default)]
    pub radical: Vec<Radical>,
    #[serde(default)]
    pub x_part: Vec<XPart>,
}

impl Into<KigouSource> for KanjiJson{
    fn into(self) -> KigouSource{
        let mut kanji: Vec<Kigou> = Vec::new();
        kanji.extend(
           self.kanji.iter().map(|v| v.into())
        );
        kanji.extend(
            self.radical.iter().map(|v| v.into())
        );
        kanji.extend(
            self.x_part.iter().map(|v| v.into())
        );
        KigouSource{kigou: kanji}
    }
}