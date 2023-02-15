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

struct Request {
    method: Method,
    path: String,
    // Sometimes receiving query parameter with HTTP request is optional 
    query_string: Option<String>, // Option<T> - Option is generic over Some(T), where T is type. Enabling space allocation ahead of time
}
