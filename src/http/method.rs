pub enum Method {
    GET(String),
    HEAD,
    POST,
    PUT,
    DELETE(u64),
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}