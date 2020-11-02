
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct GenericError {
    pub error: String
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.error)
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.error
    }
}