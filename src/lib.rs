use std::error::Error;
use std::fs;
mod value_objects;
use value_objects::*;

pub struct KanjiParser {
    parsed_kanji: Vec<ParsedKanjiJsonElement>,
}

impl KanjiParser{

    pub fn parse_kanji_json(&mut self, kanji_file_path: &str) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(kanji_file_path)?;
        self.parsed_kanji = serde_json::from_str(&contents)?;
        Ok(())
    }

    pub fn get_children(&self, identifier: &str) -> Result<Vec<&ParsedKanjiJsonElement>, Box<dyn Error>>{
        let children: Vec<&ParsedKanjiJsonElement> = self.parsed_kanji.iter()
            .filter(|x| x.parent_names.contains(&String::from(identifier)))
            .collect();
        Ok(children)
    }

    pub fn new() -> KanjiParser{
        KanjiParser{
            parsed_kanji: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_children_should_return_children(){
        let mut kanji_parser = KanjiParser::new();
        kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        let children = kanji_parser.get_children("One").unwrap();

        let kanji1 = ParsedKanjiJsonElement{
                name: String::from("Two"),
                node_type: NodeType::Kanji,
                character: String::from("二"),
                stroke_arrangement: String::from("2H"),
                stroke_count: 2,
                parent_names: vec![String::from("One")]
            };
        let kanji2 = ParsedKanjiJsonElement{
                name: String::from("Three"),
                node_type: NodeType::Kanji,
                character: String::from("三"),
                stroke_arrangement: String::from("3H"),
                stroke_count: 3,
                parent_names: vec![String::from("One"), String::from("Two")]
            };

        let expected_children = vec![&kanji1, &kanji2];
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

        kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

        assert_eq!(3, kanji_parser.parsed_kanji.len());
    }

    #[test]
    fn parse_kanji_json_should_return_expected_result(){
        let mut kanji_parser = KanjiParser::new();

        kanji_parser.parse_kanji_json("kanji_test_with_three_kanji.json").unwrap();

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

        assert_eq!(expected_parsed_kanji_json_elements, kanji_parser.parsed_kanji);
    }

}