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

/* Handling HTTP requests and return HTTP responses
 *
 * Request:
 * <METHOD> /<path>?<query_string> <protocol>
 * GET /user?id=10 HTTP/1.1 \r\n
 * HEADERS \r\n
 * BODY
 */
struct _Request {
    method: Method, // must predefine HTTP method
    path: String,
    query_string: String,
}

enum Method {
    // Enum variant also can associate with types
    GET(String), // 0
    HEAD,        // 1
    POST,        // 2
    PUT,
    DELETE(u64),
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
