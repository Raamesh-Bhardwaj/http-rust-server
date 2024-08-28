use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_params: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}


