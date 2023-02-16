/*
 * No need to use std::option::Option;
 * It's automatically loaded as default when using Option
 */

/*
 * Rust never support null value
 * Option enum have two variants which are None or Some value
 * None - expressing the absence of value but never null to avoid null pointer
 * Some - expressing the value
 */
fn main() {
    let _get = Method::GET("abcd".to_string());
    let _delete = Method::DELETE(1);
    let _post = Method::POST;
    let _put = Method::PUT;
    let _patch = Method::PATCH;
    let _head = Method::HEAD;
    let _options = Method::OPTIONS;
    let _trace = Method::TRACE;
    let _connect = Method::CONNECT;
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }
    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct _Request {
    method: Method,
    path: String,
    // Sometimes receiving query parameter with HTTP request is optional
    query_string: Option<String>, // Option<T> - Option is generic over Some(T), where T is type. Enabling space allocation ahead of time
}

enum Method {
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
