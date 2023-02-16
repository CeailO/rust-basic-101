use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

/*
 * 1. Create TCP socket and bind to the server address
 * std library provides abstraction for TCP communication
 * https://doc.rust-lang.org/std/net/index.html
 * 
 * TcpListener - listen to new connection
 * TcpStream - establish single connection for the server
 */
fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
