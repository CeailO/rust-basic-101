fn main() {
    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE(1);
    let post = Method::POST;
    let put = Method::PUT;
    let patch = Method::PATCH;
    let head = Method::HEAD;
    let options = Method::OPTIONS;
    let trace = Method::TRACE;
    let connect = Method::CONNECT;
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
struct Request {
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
