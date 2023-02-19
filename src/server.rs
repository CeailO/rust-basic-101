use std::{io::Read, net::TcpListener};

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

                    // read() return Result wrapping usize type
                    match stream.read(&mut buffer) {
                        // Ok(T) cases
                        Ok(_) => {
                            /*
                             * Logging the incoming request to the server.
                             *
                             * String struct have associate function, from_utf8() and expecting Vec<u8> (vector of bytes) from its parameter
                             * The from_utf8() will return Result wrapping String and FromUtf8Error, Result<String, FromUtf8Error>
                             *
                             * Knowledge:
                             * String is constructed from vector of bytes vec<u8>, always guaranteed to be utf-8 sequences
                             * String slice is constructed from slice of bytes &[u8], not all slices are valid utf-8 sequences. Also it's regard as primitive string.
                             *
                             * In this case, validity of utf-8 is not a concern
                             * from_utf8_lossy() is an associate function that will not fail for bytes into string conversions.
                             * Any invalid utf-8 sequences will result in U+FFFD
                             */
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            /*
                             * Further walkthrough available using Windows OS.
                             * Netcat(ncat) is require upon this test, the package can be obtained from Nmap installation
                             *
                             * To test the incoming connection request.
                             * Additional knowledge: | sign for 'pipe through'
                             * `cargo run` and `echo "TEST" | ncat 127.0.0.1 8080`
                             * or `echo TEST | ncat 127.0.0.1 8080` on two separate terminals.
                             *
                             * or by browsing through browser
                             * `localhost:8080` or `127.0.0.1:8080`
                             *
                             * More info: https://www.rfc-editor.org/rfc/rfc9110.html#section-3.9
                             * Output:
                             * Received a request: <request-method> <page_route> <http_protocol>
                             * <header_info>
                             * <empty_body>
                             */
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
