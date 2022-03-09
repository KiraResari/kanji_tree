
use serde::Deserialize;
use super::kigou_type::KigouType;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Kigou{
    pub name: String,
    pub kigou_type: KigouType,
    pub character: String,
    pub stroke_arrangement: String,
    pub stroke_count: u8,
    #[serde(default)]
    pub parent_names: Vec<String>,
    pub image_name: String,
}

impl Kigou{
    pub fn create_error_kigou(error_message: &str) -> Kigou{
        Kigou { 
            name: error_message.to_string(),
            kigou_type: KigouType::Error,
            character: error_message.to_string(), 
            stroke_arrangement: "".to_string(), 
            stroke_count: 0, 
            parent_names: Vec::new(),
            image_name: "".to_string(), 
        }
    }

    pub fn uses_image(&self) -> bool{
        self.image_name != ""
    }
}

impl ToString for Kigou{
    fn to_string(&self) -> String{
        if self.character == "" {
            return self.name.clone()
        }
        self.character.clone()
    }
}