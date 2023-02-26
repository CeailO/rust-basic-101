use std::{
    convert::{TryFrom, TryInto},
    io::Read,
    net::TcpListener,
};
/*
 * Need to pull the request scope here, crate pull the entire crates of src.
 */
use crate::http::{request, Request};

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            /*
                             * Take String as example. String struct implement std::fmt::Display trait and Debug trait
                             * thus it can be displayed in such below using println!() macros
                             *
                             * {} is implementation of std::fmt::Display
                             * {:?} is implementation of the Debug trait
                             */
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    let res: &Result<Request, _> = &buffer[..].try_into();
                                }
                                Err(e) => println!("Failed to parse a request: {}", e),
                                _ => {}
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
