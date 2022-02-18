
use std::{error::Error, io::ErrorKind};

use super::value_objects::*;

pub struct KanjiSource {
    pub kanji: Vec<Kanji>,
}

impl KanjiSource{

    pub fn get_children(&self, identifier: &str)
         -> Result<Vec<&Kanji>, Box<dyn Error>>{
        let children: Vec<&Kanji> = self.kanji.iter()
            .filter(
                |element| element.parent_names.contains(
                    &String::from(identifier)
                )
            )
            .collect();
        Ok(children)
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

    pub fn new(kanji: Vec<Kanji>) -> KanjiSource{
        KanjiSource{ kanji }
    }
}