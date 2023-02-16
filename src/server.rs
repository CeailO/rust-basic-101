use std::net::TcpListener;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        /*
         * Pass the reference to the address
         * Hold the method to read the definition
         * bind() method will return Result that wrap the TcpListener
         */
        let _listener = TcpListener::bind(&self.addr);
    }
}
