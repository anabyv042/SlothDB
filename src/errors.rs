use std::fmt;

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

impl ParsingError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParsingError: {}", self.message)
    }
}

impl From<String> for ParsingError {
    fn from(error: String) -> Self {
        ParsingError::new(&error)
    }
}
