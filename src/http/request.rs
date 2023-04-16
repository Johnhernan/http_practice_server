use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::{ Debug, Formatter, Result as FmtResult};

pub struct Request {
    path: String, 
    query_string: Option<String>, 
    method: Method
}
// Body 
impl Request {   
    fn from_byte_array( buff: &[u8]) -> Result<Self, String> {
      unimplemented!()
    }

    pub fn new(path: String, query_string: Option<String>, method: Method) -> Self {
        Request {
            path,
            query_string,
            method
        }
    }
}

// Traits 
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError { 
    InvalidRequest, 
    InvalidEncoding,
    InvalidProtocol, 
    InvalidMethod
}
// Body 
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest", 
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol", 
            Self::InvalidMethod => "InvalidMethod"
        }
    }
}
// Traits 
impl Error for ParseError {}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}