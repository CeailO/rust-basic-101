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
                    // HTTP request representation in form of buffer with bytes (1024bytes)
                    let mut buffer = [0; 1024];

                    // Need to convert the array to http request, thus need to made implementation for request struct in http/request.rs

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            /*
                             * Since TryFrom is in generic form so it can't figure [u8, 1024], buffer is [u8; 1024].
                             * We need to convert [u8, 1024] bytes slices [u8]
                             *
                             * Two ways:
                             * 1. use as keyword, so it treat it as byte slice, [u8]
                             */
                            // Request::try_from(&buffer as &[u8]);
                            /* 2. directly create slice that contains entire array by ommiting lower and upper bound */
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    /* Convert slice bytes
                                     * Can't infer type directly because of many TryInto implementations with try_into() associate function.
                                     * Thus need to explicit the type from defined variable
                                     *
                                     * We need U as U : TryForm<T> implementation
                                     * try_into() will wrap U and return it as Result<U,U::Error>
                                     */
                                    let res: &Result<Request, _> = &buffer[..].try_into();
                                }
                                /* logged the error */
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
