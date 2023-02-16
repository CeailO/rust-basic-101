use super::method::Method;
pub struct Request {
    // super refer to parent or one level up module which is http
    method: Method,
    path: String,
    query_string: Option<String>,
}
