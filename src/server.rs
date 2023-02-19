use std::net::TcpListener;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    /*
     * Example return type in form of tuple
     * pub fn run(self) -> (i32, &'static str, std::net::TcpListener)
     */
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // accept() is used to receive incoming connections from TcpListener
        loop {
            /*
             * accept() return Result wrapping two type simultaneously which are
             * TcpStream and SocketAddr (address of the client)
             *
             * Concept of tuples
             * General way to group a number of values with different types into a compound type
             * Tuples are fixed in size, it can't grow or shrink in size
             *
             * From Result<(TcpStream, SocketAddr)>, (TcpStream, SocketAddr) is a tuples
             * which have TcpStream type and SocketAddr type
             */
            listener.accept(); // Continue the loop once it receive the connection
        }
        /*
         * Syntax defining tuples
         * (<element_type_1>, <element_type_2>, <element_type_3)
         */
        // let tup = (5, "a", listener);

        // Example for returning a tuple
        // (5, "a", listener)
    }
}
