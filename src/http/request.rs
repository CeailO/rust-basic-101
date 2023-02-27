use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, from_utf8, Utf8Error};

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}

pub enum ParseError {
    InvalidMethod,
    InvalidRequest,
    // utf-8 invalid error
    InvalidEncoding,
    InvalidProtocol,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidMethod => "Invalid method",
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
        }
    }
}

/*
 * implementation of From<Utf8Error> trait to match TryForm<&[u8]> format
 *      #[stable(feature = "rust1", since = "1.0.0")]
 *      fn from(value: T) -> Self;
 */
impl From<Utf8Error> for ParseError {
    /*
     * Receive Utf8Error argument
     * Return ParseError
     */
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    /*
     * HTTP request
     * <http_method> /<path>?<query_string> <http_protocol>
     * GET /search?name=abc&sort=1 HTTP/1.1
     */

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
         * Converting buffer to Request Type by converting &[u8] to String
         * Ensuring buffer as valid utf-8 character
         *
         * Result<&str, Utf8Error> thus using match to unwrap it
         *
         * Two ways to obtained Result<&str, Utf8Error> type
         * 1. without `or()`
         */
        // match from_utf8(buf) {
        //     Ok(_request) => {
        //         todo!()
        //     }
        //     // return utf-8 encoding error variant
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        /*
         * 2. Using `or()` associate funtion (common pattern)
         *
         * if success, or() will return Ok() to the self function, `from_utf8()` in this case.
         * if failed, or() will return Err() argument `ParseError::InvalidEncoding` in this case
         */
        // match from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(_) => {}
        //     Err(e) => return Err(e),
        // }

        /* Instead of `match` above, the same `match` statement can be represented as below */
        // let request = from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        /*
         * error occured as it will try to return Utf8Error type instead of ParseError type if there's
         * no trait implementation of Utf8Error in ParseError
         */
        let _request = from_utf8(buf)?;

        // let string = String::from("asd");
        // string.encrpyt();
        // buf.encrpyt();

        unimplemented!()
    }
}

trait Encrypt {
    fn encrpyt(&self) -> Self;
}

impl Encrypt for String {
    fn encrpyt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrpyt(&self) -> Self {
        unimplemented!()
    }
}
