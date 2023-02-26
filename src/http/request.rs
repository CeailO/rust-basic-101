use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}

// impl Request {
/*
 * Function that accepts buffer in form of slice of bytes and return Request struct
 *
 * Cases need to be factorised:
 * 1. Accepted invalid bytes request
 * 2. Unable to convert to Request struct because of invalid http request
 *
 * from_byte_array have chances to fail.
 * Failed computation need to return Result
 *
 * https://doc.rust-lang.org/std/convert/index.html
 * Traits are similar as interface in other languages
 * Implementing From traits, bytes array (in this case slice of bytes) to request.
 * The traits should not fail. If failed, use TryForm traits
 * https://doc.rust-lang.org/std/convert/trait.TryFrom.html
 */
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

/*
 * Trait block syntax:
 * impl <trait_name_from_library> for <targeted_struct_block> {}
 *
 * Based on TryFrom implementation traits module:
 * TryForm is generic over type T, TryFrom<T>
 * T refer to the type we are trying to convert from (in this case, byte array or slice of byte)
 * It will return Result containing the Self (targeted struct)
 *
 * There's type alias (new name for existing type)
 *
 * The structure of the code turn into idiomatic rust
 *
 * https://doc.rust-lang.org/std/convert/trait.TryFrom.html
 * implementing TryForm<T> for U will cause the compiler automatically will try ahead to generate the code
 * from TryInto<U> for T from other crate.
 */
impl TryFrom<&[u8]> for Request {
    // Need to implement 'type Error'
    type Error = String;
    // Need to implement 'try_from' methods
    
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
         * Usage case from extension implementation of encrypt for String and slice of byte
         */
        // let string = String::from("asd");
        // string.encrpyt();
        // buf.encrpyt();

        /*
         * Calling encryption function for String type
         */
        let string = String::from("asd");
        string.encrpyt();

        /*
         * Calling encryption function for bytes slice
         * accepting buf argument through function parameter
         */
        buf.encrpyt();

        /*
         * Need return value
         * unimplemented!() a macro for unimplemented function yet to supress the complier complaints.
         */
        unimplemented!()
    }
}

/*
 * Creating custom traits for encryption
 *
 * This trait does look at internal state
 * Afterwards, encrypt and later return
 * New instances of itself
 */

trait Encrypt {
    // Assessing internal state, encrypt and return new instance itself
    fn encrpyt(&self) -> Self;
}

/*
 * Implement Encrypt and extending the String from external, std
 * Imported crate function also can use the custom trait
 */
impl Encrypt for String {
    fn encrpyt(&self) -> Self {
        unimplemented!()
    }
}

/*
 * Implement Encrypt and extending the &[u8] from external, std
 */
impl Encrypt for &[u8] {
    fn encrpyt(&self) -> Self {
        unimplemented!()
    }
}
