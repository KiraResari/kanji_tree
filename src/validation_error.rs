use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ValidationError {
    description: String
}

impl ValidationError {
    pub fn new(description: String) -> ValidationError {
        ValidationError{description}
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.description)
    }
}

impl Error for ValidationError {
    fn description(&self) -> &str {
        &self.description
    }
}