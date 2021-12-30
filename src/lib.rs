use std::error::Error;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ParsedKanjiJsonElement{
    pub name: String,
    pub node_type: NodeType,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum NodeType {
    Kanji,
    Radical,
    XPart,
    Dead,
}

pub fn parse_kanji_json(kanji_file_path: &str) -> Result<Vec<ParsedKanjiJsonElement>, Box<dyn Error>>{
    let contents = fs::read_to_string(kanji_file_path)?;
    let parsed_kanji: Vec<ParsedKanjiJsonElement> = serde_json::from_str(&contents)?;
    Ok(parsed_kanji)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_kanji_json_should_not_return_error(){
        match parse_kanji_json("kanji.json"){
            Ok(_) => println!("Test Passed"),
            Err(error) => panic!("{}", error),
        }
    }

    #[test]
    fn parse_kanji_json_should_return_correct_count_of_kanji(){
        let parsed_kanji: Vec<ParsedKanjiJsonElement>
            = parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        assert_eq!(3, parsed_kanji.len());
    }

    #[test]
    fn parse_kanji_json_should_return_expected_result(){
        let parsed_kanji: Vec<ParsedKanjiJsonElement>
            = parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        let expected_parsed_kanji_json_elements = vec![
            ParsedKanjiJsonElement{
                name: String::from("One"),
                node_type: NodeType::Kanji,
                character: String::from("一"),
                stroke_arrangement: String::from("Whole"),
                stroke_count: 1,
                parent_names: vec![]
            },
            ParsedKanjiJsonElement{
                name: String::from("Two"),
                node_type: NodeType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")]
            },
            ParsedKanjiJsonElement{
                name: String::from("Three"),
                node_type: NodeType::Kanji,
                character: String::from("三"),
                stroke_arrangement: String::from("3H"),
                stroke_count: 3,
                parent_names: vec![String::from("One"), String::from("Two")]
            }
        ];

        assert_eq!(expected_parsed_kanji_json_elements, parsed_kanji);
    }

}