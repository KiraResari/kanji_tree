
use std::{io::ErrorKind};
use std::error::Error;

use super::value_objects::*;

#[derive(Clone)]
pub struct KigouSource {
    pub kigou: Vec<Kigou>,
}

impl KigouSource{

    pub fn get_children(&self, name: &String)
         -> Vec<Kigou>{
        self.kigou.iter()
            .filter(
                |element| element.parent_names.contains(
                    name
                )
            ).cloned()
            .collect()
    }

    pub fn has_children(&self, name: &String) -> bool{
        let children = self.get_children(name);
        children.len() > 0
    }

    pub fn get_parents(&self, name: &str)
        -> Vec<Kigou>{
        let query_element_option = self.kigou.iter()
            .find(|element| element.name == name);
        let query_element: &Kigou;
        match query_element_option{
            Some(v) => query_element = v,
            None => return vec![]
        }
        let parents: Vec<Kigou> = self.kigou.iter()
        .filter(
            |element| query_element.parent_names.contains(&element.name)
        ).cloned()
        .collect();
        parents
    }

    pub fn get_kigou_by_name(&self, name: &str) -> Option<&Kigou>{
        self.kigou
            .iter()
            .find(|element| element.name.to_lowercase() == name.to_lowercase())
    }

    pub fn get_kigou_by_name_fuzzy(&self, name: &str) -> Option<&Kigou>{
        self.kigou
            .iter()
            .find(
                |element|
                element.name.to_lowercase().contains(&name.to_lowercase())
            )
    }

    pub fn get_kigou_by_character(&self, character: &str) -> Option<&Kigou>{
        self.kigou.iter().find(|element| element.character == character)
    }

    pub fn get_first_element(&self)
        -> Result<Kigou, std::io::Error>{
        let first_element_option = self.kigou.first();
        match first_element_option{
            Some(v) => Ok(v.clone()),
            None =>{
                Err(
                    std::io::Error::new(
                        ErrorKind::Other,
                        "Kanji list contains no elements".to_string() 
                    )
                )
            }
        }
    }

    pub fn create_kigou_source_for_invalid_json(error: Box<dyn Error>)
     -> KigouSource{
        let error_message = error.to_string();
        let error_kigou = Kigou::create_error_kigou(&error_message);
        KigouSource{
            kigou: vec![error_kigou],
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::{value_objects::{Kigou, KigouType}, kigou_parser::KigouParser};

    use super::*;

    #[test]
    fn get_children_should_return_children(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let children = kanji_source.get_children(
            &"One".to_string()
        );

        let kanji_two = Kigou{
                name: String::from("Two"),
                kigou_type: KigouType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")],
                image_name: "".to_string(),
            };
        let kanji_three = Kigou{
                name: String::from("Three"),
                kigou_type: KigouType::Kanji,
                character: String::from("三"),
                stroke_arrangement: String::from("3H"),
                stroke_count: 3,
                parent_names: vec![String::from("One"), String::from("Two")],
                image_name: "".to_string(),
            };

        let expected_children = vec![kanji_two, kanji_three];
        assert_eq!(children, expected_children);
    }

    fn get_kigou_source_from_test_file(file_name: &str) -> KigouSource {
        let mut kigou_parser = KigouParser::new();
        let kanji_source: KigouSource
             = kigou_parser.parse_kanji_json(
                 &format!("{}{}", "resources/test/", file_name)
            ).unwrap();
        kanji_source
    }

    #[test]
    fn get_element_should_return_element(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let element = kanji_source.get_kigou_by_name("Two").unwrap();

        let kanji_two = Kigou{
                name: String::from("Two"),
                kigou_type: KigouType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")],
                image_name: "".to_string(),
            };

        assert_eq!(element, &kanji_two);
    }

    #[test]
    fn get_element_should_return_none_if_element_does_not_exist(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let result = kanji_source.get_kigou_by_name("Does Not Exist");

        assert_eq!(result, None);
    }

    #[test]
    fn get_parents_should_return_parents(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let parents = kanji_source.get_parents("Three");

        let kanji_one = Kigou{
            name: String::from("One"),
            kigou_type: KigouType::Kanji,
            character: String::from("一"),
            stroke_arrangement: String::from("Whole"),
            stroke_count: 1,
            parent_names: vec![],
            image_name: "".to_string(),
        };
        let kanji_two = Kigou{
            name: String::from("Two"),
            kigou_type: KigouType::Kanji,
            character: String::from("二"),
            stroke_arrangement: String::from("2H"),
            stroke_count: 2,
            parent_names: vec![String::from("One")],
            image_name: "".to_string(),
        };


    let expected_children = vec![kanji_one, kanji_two];
    assert_eq!(parents, expected_children);
    }

    #[test]
    fn get_first_element_should_return_first_element(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let element = kanji_source.get_first_element().unwrap();

        let kanji_one = Kigou{
            name: String::from("One"),
            kigou_type: KigouType::Kanji,
            character: String::from("一"),
            stroke_arrangement: String::from("Whole"),
            stroke_count: 1,
            parent_names: vec![],
            image_name: "".to_string(),
        };

        assert_eq!(element, kanji_one);
    }

    #[test]
    #[should_panic(expected = "Kanji list contains no elements")]
    fn get_first_element_should_return_error_if_kanji_list_is_empty(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_empty.json"
        );

        kanji_source.get_first_element().unwrap();
    }

    #[test]
    fn get_kigou_by_chracter_should_return_correct_kigou(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let element = kanji_source.get_kigou_by_character("二").unwrap();

        let kanji_two = Kigou{
                name: String::from("Two"),
                kigou_type: KigouType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")],
                image_name: "".to_string(),
            };

        assert_eq!(element, &kanji_two);
    }

    #[test]
    fn get_kigou_by_name_should_be_case_insensitive(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let element = kanji_source.get_kigou_by_name("two").unwrap();

        let kanji_two = Kigou{
                name: String::from("Two"),
                kigou_type: KigouType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")],
                image_name: "".to_string(),
            };

        assert_eq!(element, &kanji_two);
    }

    #[test]
    fn get_kigou_by_name_fuzzy_should_return_correct_kigou(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let element = kanji_source.get_kigou_by_name_fuzzy("tw").unwrap();

        let kanji_two = Kigou{
                name: String::from("Two"),
                kigou_type: KigouType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")],
                image_name: "".to_string(),
            };

        assert_eq!(element, &kanji_two);
    }

    #[test]
    fn has_children_should_return_true_for_one(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let result = kanji_source.has_children(&"One".to_string());

        assert_eq!(result, true);
    }

    #[test]
    fn has_children_should_return_false_for_three(){
        let kanji_source = get_kigou_source_from_test_file(
            "kanji_test_with_three_kanji.json"
        );

        let result = kanji_source.has_children(&"Three".to_string());

        assert_eq!(result, false);
    }

}