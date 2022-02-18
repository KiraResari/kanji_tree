mod app;
use std::error::Error;
use std::fs;
mod value_objects;
pub use app::KanjiTreeApp;
mod kanji_source; 
use kanji_source::KanjiSource;

pub struct KanjiParser {
}

impl KanjiParser{

    pub fn parse_kanji_json(&mut self, kanji_file_path: &str)
         -> Result<KanjiSource, Box<dyn Error>>{
        let contents = fs::read_to_string(kanji_file_path)?;
        let parsed_kanji = serde_json::from_str(&contents)?;
        Ok(KanjiSource::new(parsed_kanji))
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
    fn get_children_should_return_children(){
        let mut kanji_parser = KanjiParser::new();
        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json(
                 "kanji_test_with_three_kanji.json"
            ).unwrap();

        let children = kanji_source.get_children("One").unwrap();

        let kanji_two = Kanji{
                name: String::from("Two"),
                node_type: NodeType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")]
            };
        let kanji_three = Kanji{
                name: String::from("Three"),
                node_type: NodeType::Kanji,
                character: String::from("三"),
                stroke_arrangement: String::from("3H"),
                stroke_count: 3,
                parent_names: vec![String::from("One"), String::from("Two")]
            };

        let expected_children = vec![&kanji_two, &kanji_three];
        assert_eq!(children, expected_children);
    }

    #[test]
    fn get_element_should_return_element(){
        let mut kanji_parser = KanjiParser::new();
        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        let element = kanji_source.get_element("Two").unwrap();

        let kanji_two = Kanji{
                name: String::from("Two"),
                node_type: NodeType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")]
            };

        assert_eq!(element, &kanji_two);
    }

    #[test]
    #[should_panic(expected = "No Kanji with name")]
    fn get_element_should_return_error_if_element_does_not_exist(){
        let mut kanji_parser = KanjiParser::new();
        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        kanji_source.get_element("Does Not Exist").unwrap();
    }

    #[test]
    fn get_parents_should_return_parents(){
        let mut kanji_parser = KanjiParser::new();
        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        let children = kanji_source.get_parents("Three").unwrap();

        let kanji_one = Kanji{
            name: String::from("One"),
            node_type: NodeType::Kanji,
            character: String::from("一"),
            stroke_arrangement: String::from("Whole"),
            stroke_count: 1,
            parent_names: vec![]
        };
        let kanji_two = Kanji{
            name: String::from("Two"),
            node_type: NodeType::Kanji,
            character: String::from("二"),
            stroke_arrangement: String::from("2H"),
            stroke_count: 2,
            parent_names: vec![String::from("One")]
        };


    let expected_children = vec![&kanji_one, &kanji_two];
    assert_eq!(children, expected_children);
    }

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

}