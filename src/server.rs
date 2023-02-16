/*
 * Every files in rust is treated as modules
 * so no need encapsulate stuffs into mod server {}
 * Items inside modules is private by default for both types and functionality
 */

// Type
pub struct Server {
    addr: String,
}
impl Server {
    // Funtionality
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
