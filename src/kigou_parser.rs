
use std::error::Error;
use std::fs;
use crate::kigou_source::KigouSource;
use crate::value_objects::KanjiJson;

pub struct KigouParser {
}

impl KigouParser{

    pub fn parse_kanji_json(&mut self, kanji_file_path: &str)
    -> Result<KigouSource, Box<dyn Error>>{
        let contents = fs::read_to_string(kanji_file_path)?;
        let kanji_json:KanjiJson = serde_json::from_str(&contents)?;
        Ok(kanji_json.into())
    }

    pub fn new() -> KigouParser{
        KigouParser{ }
    }
}

#[cfg(test)]
mod tests {
    use crate::value_objects::{Kigou, KigouType};

    use super::*;

    #[test]
    fn parse_kanji_json_should_not_return_error(){
        let mut kanji_parser = KigouParser::new();

        match kanji_parser.parse_kanji_json("kanji.json"){
            Ok(_) => println!("Test Passed"),
            Err(error) => panic!("{}", error),
        }
    }

    #[test]
    fn parse_kanji_json_should_return_correct_count_of_kanji(){
        let mut kanji_parser = KigouParser::new();

        let kanji_source: KigouSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        assert_eq!(3, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_should_return_expected_result(){
        let mut kanji_parser = KigouParser::new();

        let kanji_source: KigouSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        let expected_parsed_kanji_json_elements = vec![
            Kigou{
                name: String::from("One"),
                kigou_type: KigouType::Kanji,
                character: String::from("一"),
                stroke_arrangement: String::from("Whole"),
                stroke_count: 1,
                parent_names: vec![],
                image_name: "".to_string(),
            },
            Kigou{
                name: String::from("Two"),
                kigou_type: KigouType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")],
                image_name: "".to_string(),
            },
            Kigou{
                name: String::from("Three"),
                kigou_type: KigouType::Kanji,
                character: String::from("三"),
                stroke_arrangement: String::from("3H"),
                stroke_count: 3,
                parent_names: vec![String::from("One"), String::from("Two")],
                image_name: "".to_string(),
            }
        ];

        assert_eq!(expected_parsed_kanji_json_elements, kanji_source.kigou);
    }

    #[test]
    fn parse_kanji_json_with_separate_sections_should_not_return_error(){
        let mut kanji_parser = KigouParser::new();

        match kanji_parser.parse_kanji_json(
            "kanji_test_with_separate_kanji_and_radical.json"
        ){
            Ok(_) => println!("Test Passed"),
            Err(error) => panic!("{}", error),
        }
    }

    #[test]
    fn parse_kanji_json_with_separate_sections_should_return_correct_count_of_kanji(){
        let mut kanji_parser = KigouParser::new();

        let kanji_source: KigouSource
             = kanji_parser.parse_kanji_json(
                 "kanji_test_with_separate_kanji_and_radical.json"
            ).unwrap();

        assert_eq!(3, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_with_x_part_should_return_correct_count_of_kigou(){
        let mut kigou_parser = KigouParser::new();

        let kanji_source: KigouSource
             = kigou_parser.parse_kanji_json(
                 "kanji_test_with_x_part.json"
            ).unwrap();

        assert_eq!(1, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_with_kana_should_return_correct_count_of_kigou(){
        let mut kigou_parser = KigouParser::new();

        let kanji_source: KigouSource
             = kigou_parser.parse_kanji_json(
                 "kanji_test_with_kana.json"
            ).unwrap();

        assert_eq!(1, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_with_dead_kanji_should_return_correct_count_of_kigou(){
        let mut kigou_parser = KigouParser::new();

        let kanji_source: KigouSource
             = kigou_parser.parse_kanji_json(
                 "kanji_test_with_dead_kanji.json"
            ).unwrap();

        assert_eq!(1, kanji_source.kigou.len());
    }

}