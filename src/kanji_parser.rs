
use std::error::Error;
use std::fs;
use crate::kanji_source::KanjiSource;
use crate::value_objects::{NodeContainer, Kanji};

pub struct KanjiParser {
}

impl KanjiParser{

    pub fn parse_kanji_json(&mut self, kanji_file_path: &str)
         -> Result<KanjiSource, Box<dyn Error>>{
        let contents = fs::read_to_string(kanji_file_path)?;
        let parsed_kanji = serde_json::from_str(&contents)?;
        Ok(KanjiSource::new(parsed_kanji))
    }

    pub fn parse_kanji_json_with_separate_sections(&mut self, kanji_file_path: &str)
    -> Result<KanjiSource, Box<dyn Error>>{
        let contents = fs::read_to_string(kanji_file_path)?;
        let parsed_node_container:NodeContainer = serde_json::from_str(&contents)?;
        Ok(parsed_node_container.into())
    }

    pub fn new() -> KanjiParser{
        KanjiParser{ }
    }
}

#[cfg(test)]
mod tests {
    use crate::value_objects::{Kanji, NodeType};

    use super::*;

    #[test]
    fn parse_kanji_json_should_not_return_error(){
        let mut kanji_parser = KanjiParser::new();

        match kanji_parser.parse_kanji_json("kanji.json"){
            Ok(_) => println!("Test Passed"),
            Err(error) => panic!("{}", error),
        }
    }

    #[test]
    fn parse_kanji_json_should_return_correct_count_of_kanji(){
        let mut kanji_parser = KanjiParser::new();

        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        assert_eq!(3, kanji_source.kanji.len());
    }

    #[test]
    fn parse_kanji_json_should_return_expected_result(){
        let mut kanji_parser = KanjiParser::new();

        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        let expected_parsed_kanji_json_elements = vec![
            Kanji{
                name: String::from("One"),
                node_type: NodeType::Kanji,
                character: String::from("一"),
                stroke_arrangement: String::from("Whole"),
                stroke_count: 1,
                parent_names: vec![]
            },
            Kanji{
                name: String::from("Two"),
                node_type: NodeType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")]
            },
            Kanji{
                name: String::from("Three"),
                node_type: NodeType::Kanji,
                character: String::from("三"),
                stroke_arrangement: String::from("3H"),
                stroke_count: 3,
                parent_names: vec![String::from("One"), String::from("Two")]
            }
        ];

        assert_eq!(expected_parsed_kanji_json_elements, kanji_source.kanji);
    }

    #[test]
    fn parse_kanji_json_with_separate_sections_should_not_return_error(){
        let mut kanji_parser = KanjiParser::new();

        match kanji_parser.parse_kanji_json_with_separate_sections(
            "kanji_test_with_separate_kanji_and_radical.json"
        ){
            Ok(_) => println!("Test Passed"),
            Err(error) => panic!("{}", error),
        }
    }

    #[test]
    fn parse_kanji_json_with_separate_sections_should_return_correct_count_of_kanji(){
        let mut kanji_parser = KanjiParser::new();

        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json_with_separate_sections(
                 "kanji_test_with_separate_kanji_and_radical.json"
            ).unwrap();

        assert_eq!(3, kanji_source.kanji.len());
    }

}