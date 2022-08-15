
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::kigou_source::KigouSource;
use crate::value_objects::KanjiJson;
use crate::validation_error::ValidationError;

pub struct KigouParser {
}

impl KigouParser{

    pub fn parse_kanji_json(&mut self, kanji_file_path: &str)
    -> Result<KigouSource, Box<dyn Error>>{
        let contents = fs::read_to_string(kanji_file_path)?;
        let kanji_json:KanjiJson = serde_json::from_str(&contents)?;
        let kigou_source:KigouSource = kanji_json.into();
        KigouParser::validate(kigou_source)
    }

    pub fn new() -> KigouParser{
        KigouParser{ }
    }

    fn validate(kigou_source: KigouSource) -> Result<KigouSource, Box<dyn Error>>{
        let duplicate_names = KigouParser::find_duplicate_names(kigou_source.clone());
        if duplicate_names.len() > 0{
            return Err(
                Box::new(
                    ValidationError::new(
                        format!(
                            "kanji.json contains duplicate names: {:?}",
                            duplicate_names)
                        )
                    )
                )
        }
        let dead_parents= KigouParser::find_dead_parents(kigou_source.clone());
        if dead_parents.len() > 0{
            return Err(
                Box::new(
                    ValidationError::new(
                        format!(
                            "kanji.json contains dead parents: {:?}",
                            dead_parents)
                        )
                    )
                )
        }
        let missing_images= KigouParser::find_missing_images(kigou_source.clone());
        if missing_images.len() > 0{
            return Err(
                Box::new(
                    ValidationError::new(
                        format!(
                            "kanji.json contains missing images: {:?}",
                            missing_images)
                        )
                    )
                )
        }
        Ok(kigou_source)
    }

    fn find_duplicate_names(kigou_source: KigouSource) -> Vec<String>{
        let mut duplicate_names: Vec<String> = Vec::new();
        let mut scanned_names: Vec<String> = Vec::new();
        let all_kigou = kigou_source.kigou;
        for kigou in all_kigou{
            if scanned_names.contains(&kigou.name){
                duplicate_names.push(kigou.name.clone());  
            }
            scanned_names.push(kigou.name);
        }
        duplicate_names
    }

    fn find_dead_parents(kigou_source: KigouSource) -> Vec<String>{
        let mut dead_parents: Vec<String> = Vec::new();
        let all_kigou = kigou_source.clone().kigou;
        for kigou in all_kigou{
            for parent_name in kigou.parent_names{
                let parent_option = kigou_source.get_kigou_by_name(&parent_name);
                match parent_option{
                    Some(_) => {},
                    None => {
                        dead_parents.push(format!("Kigou '{}' is missing parent '{}'", kigou.name, parent_name));
                    }
                }
            }
        }
        dead_parents
    }

    fn find_missing_images(kigou_source: KigouSource) -> Vec<String>{
        let mut missing_images: Vec<String> = Vec::new();
        let all_kigou = kigou_source.kigou;
        for kigou in all_kigou{
            if kigou.image_name == ""{
                continue;
            }
            if !Path::new(&format!("resources/images/{}", kigou.image_name)).exists(){
                missing_images.push(format!("Kigou '{}' can't find image '{}'", kigou.name, kigou.image_name)); 
            }
        }
        missing_images
    }

}

#[cfg(test)]
mod tests {
    use crate::value_objects::{Kigou, KigouType};

    use super::*;

    #[test]
    fn parse_kanji_json_should_not_return_error(){
        let mut kanji_parser = KigouParser::new();

        match kanji_parser.parse_kanji_json("resources/kanji.json"){
            Ok(_) => println!("Test Passed"),
            Err(error) => panic!("{}", error),
        }
    }

    #[test]
    fn parse_kanji_json_should_return_correct_count_of_kanji(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        assert_eq!(3, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_should_return_expected_result(){
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

        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        assert_eq!(expected_parsed_kanji_json_elements, kanji_source.kigou);
    }

    #[test]
    fn parse_kanji_json_with_separate_sections_should_not_return_error(){
        get_kigou_source_from_test_file(
            "kanji_test_with_separate_kanji_and_radical.json"
        );
    }

    #[test]
    fn parse_kanji_json_with_separate_sections_should_return_correct_count_of_kanji(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_separate_kanji_and_radical.json"
        );

        assert_eq!(3, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_with_x_part_should_return_correct_count_of_kigou(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_x_part.json"
        );

        assert_eq!(1, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_with_kana_should_return_correct_count_of_kigou(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_kana.json"
        );

        assert_eq!(1, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_with_dead_kanji_should_return_correct_count_of_kigou(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_dead_kanji.json"
        );

        assert_eq!(1, kanji_source.kigou.len());
    }

    #[test]
    fn parse_kanji_json_with_duplicate_name_should_return_error(){
        let result 
            = get_kigou_source_result_from_test_file("kanji_test_with_duplicate_name.json");

        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn parse_kanji_json_with_dead_parent_should_return_error(){
        let result 
            = get_kigou_source_result_from_test_file("kanji_test_with_dead_parent.json");

        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn parse_kanji_json_with_missing_image_should_return_error(){
        let result 
            = get_kigou_source_result_from_test_file("kanji_test_with_missing_image.json");

        assert!(matches!(result, Err(_)));
    }

    fn get_kigou_source_from_test_file(file_name: &str) -> KigouSource {
        get_kigou_source_result_from_test_file(file_name).unwrap()
    }

    fn get_kigou_source_result_from_test_file(file_name: &str) -> Result<KigouSource, Box<dyn Error>> {
        let mut kigou_parser = KigouParser::new();
        kigou_parser.parse_kanji_json(
            &format!("{}{}", "resources/test/", file_name)
        )
    }

}