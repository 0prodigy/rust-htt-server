use super::method::Method;
use std::convert::TryFrom

pub struct Request {
    path:String,
    query_string:Option<String>,
    method:Method,
}

impl Request {
    pub fn from_byte_array(buf: &u8[]) -> Result<Self,Error> {

    }
}

impl TryFrom<&u8[]> for Request {
    type Error = String;
    fn try_from(buf: &u8[]) -> Result<Self, Self::Error> {
       
    }
}