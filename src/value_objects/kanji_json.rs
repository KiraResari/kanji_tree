use serde::Deserialize;

use crate::kigou_source::KigouSource;

use super::{Kanji, Radical, Kigou, XPart, Kana, DeadKanji};

#[derive(Deserialize)]
pub struct KanjiJson{
    #[serde(default)]
    pub kanji: Vec<Kanji>,
    #[serde(default)]
    pub radical: Vec<Radical>,
    #[serde(default)]
    pub x_part: Vec<XPart>,
    #[serde(default)]
    pub kana: Vec<Kana>,
    #[serde(default)]
    pub dead: Vec<DeadKanji>,
}

impl Into<KigouSource> for KanjiJson{
    fn into(self) -> KigouSource{
        let mut kigou: Vec<Kigou> = Vec::new();
        kigou.extend(
           self.kanji.iter().map(|v| v.into())
        );
        kigou.extend(
            self.radical.iter().map(|v| v.into())
        );
        kigou.extend(
            self.x_part.iter().map(|v| v.into())
        );
        kigou.extend(
            self.kana.iter().map(|v| v.into())
        );
        kigou.extend(
            self.dead.iter().map(|v| v.into())
        );
        KigouSource{kigou}
    }
}