use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
/*
 * Can use curly brackets for importing same parents modules for different children
 * Casting Result using alias FmtResult to resolve default module format conflicts
 */
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}

pub enum ParseError {
    InvalidMethod,
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
}

impl ParseError {
    /* self is enum */
    fn message(&self) -> &str {
        match self {
            // The value will be return from match automatically as it written with no scope
            Self::InvalidMethod => "Invalid method",
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
        }
    }
}

impl Display for ParseError {
    /*
     * Based on Display documentation for trait implementations
     *      #[stable(feature = "rust1", since = "1.0.0")]
     *      fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;
     *
     *  Result is not generic type in this case. It's special type alias pointing to result::Result<(), Error>
     *      pub type Result = result::Result<(), Error>;
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        /*
         * Using write macro for to generate the string
         * Need to implement self.message() in ParseError
         */
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    /*
     * Based on Debug documentation for trait implementations
     *      #[stable(feature = "rust1", since = "1.0.0")]
     *      fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // same as Display implementation above
        write!(f, "{}", self.message())
    }
}

/*
 * ParseError yet to have std::fmt::Display trait and Debug trait
 */
impl Error for ParseError {}

impl TryFrom<&[u8]> for Request {
    /*
     * Custom error for ParseError variant representation
     */
    type Error = ParseError;

    /*
     * HTTP request
     * <http_method> /<path>?<query_string> <http_protocol>
     * GET /search?name=abc&sort=1 HTTP/1.1
     */

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let string = String::from("asd");
        string.encrpyt();
        buf.encrpyt();

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
