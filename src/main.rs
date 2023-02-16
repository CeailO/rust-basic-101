/*
 * Modules control visibility of items to outside
 * submodules also follow the hierarchy
 *
 * public - available for both external implementations
 * private - available for internal implementation
 */

// import modules from submodular in mod.rs
use http::Method; // Still able to use http::method::Method
use http::Request; // Still able to use http::request::Request
use server::Server;

// http.rs file
mod http;
// server.rs file
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
