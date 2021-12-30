use std::error::Error;

pub struct ParsedKanjiJsonElement{
    pub name: String,
    pub node_type: NodeType,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    pub parent_names: Vec<String>,
}

pub enum NodeType {
    Kanji,
    Radical,
    XPart,
    Dead,
}

pub fn parse_kanji_json(kanji_file_path: &str) -> Result<(), Box<dyn Error>>{
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_kanji_json_should_not_return_error() -> Result<(), Box<dyn Error>>{
        parse_kanji_json("kanji.json")
    }
}