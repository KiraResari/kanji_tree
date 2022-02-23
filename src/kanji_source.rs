
use std::{error::Error, io::ErrorKind};

use super::value_objects::*;

pub struct KanjiSource {
    pub kanji: Vec<Kanji>,
}

impl KanjiSource{

    pub fn new(kanji: Vec<Kanji>) -> KanjiSource{
        KanjiSource{ kanji }
    }

    pub fn get_children(&self, identifier: &String)
         -> Vec<Kanji>{
        self.kanji.iter()
            .filter(
                |element| element.parent_names.contains(
                    identifier
                )
            ).cloned()
            .collect()
    }

    pub fn get_parents(&self, identifier: &str)
        -> Result<Vec<&Kanji>, Box<dyn Error>>{
        let query_element_option = self.kanji.iter()
            .find(|element| element.name == identifier);
        let query_element: &Kanji;
        match query_element_option{
            Some(v) => query_element = v,
            None => return Ok(vec![])
        }
        let parents: Vec<&Kanji> = self.kanji.iter()
        .filter(
            |element| query_element.parent_names.contains(&element.name)
        )
        .collect();
        Ok(parents)
    }

    pub fn get_element(&self, identifier: &str)
        -> Result<&Kanji, std::io::Error>{
        let query_element_option = self.kanji.iter()
            .find(|element| element.name == identifier);
        match query_element_option{
            Some(v) => Ok(v),
            None =>{
                Err(
                    std::io::Error::new(
                        ErrorKind::Other,
                        format!(
                            "No Kanji with name '{}' could be found.",
                            identifier
                        )
                    )
                )
            }
        }
    }

    pub fn get_first_element(&self)
        -> Result<Kanji, std::io::Error>{
        let first_element_option = self.kanji.first();
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
}

#[cfg(test)]
mod tests {
    use crate::{value_objects::{Kanji, NodeType}, kanji_parser::KanjiParser};

    use super::*;

    #[test]
    fn get_children_should_return_children(){
        let mut kanji_parser = KanjiParser::new();
        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json(
                 "kanji_test_with_three_kanji.json"
            ).unwrap();

        let children = kanji_source.get_children(
            &"One".to_string()
        );

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

        let expected_children = vec![kanji_two, kanji_three];
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
    fn get_first_element_should_return_first_element(){
        let mut kanji_parser = KanjiParser::new();
        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        let element = kanji_source.get_first_element().unwrap();

        let kanji_one = Kanji{
            name: String::from("One"),
            node_type: NodeType::Kanji,
            character: String::from("一"),
            stroke_arrangement: String::from("Whole"),
            stroke_count: 1,
            parent_names: vec![]
        };

        assert_eq!(element, kanji_one);
    }

    #[test]
    #[should_panic(expected = "Kanji list contains no elements")]
    fn get_first_element_should_return_error_if_kanji_list_is_empty(){
        let mut kanji_parser = KanjiParser::new();
        let kanji_source: KanjiSource
             = kanji_parser.parse_kanji_json("kanji_test_empty.json").unwrap();

        kanji_source.get_first_element().unwrap();
    }

}