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
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            /* Matching on Results enums will not compile the program until all
             * the possible variants is covered
             */
            match listener.accept() {
                /*
                 * Checking the Result<(TcpStream, SocketAddr)> with first variant
                 * The variant can bind it's body value by passing it to the given name
                 *
                 * In this case variant Ok(<given_name>) {<body_value>}
                 */
                // Ok(tup) => { // Destruct tup
                // If didn't want to use the value can substitute with _
                Ok((_stream, _addr)) => {
                    // handling Ok() case
                    let a = 5;
                    println!("Ok");
                }
                Err(e) => println!("Failed to establish connection: {}", e),
                /*
                 * Catching all the variants that not manually explicit like the case above
                 * The concept is similar with switch statements and default case
                 */
                // _ => println!()
            }

            // Regular match expression
            // match "abcd" {
            //     "abcd" => {}
            //     // pattern alternative operator
            //     "a" | "b" => {}
            //     _ => println!(""),
            // }

            /*
             * In such cases if the accept() fails to accept the connection
             *
             * Recoverable error will be needed
             * We need to unwrap the Result type from accept() by using unwrap()
             * The unwrap() will return E: fmt::Debug to terminate the program
             * if accept() Result return Err(E)
             */
            // listener.accept().unwrap();

            /*
             * 1. Need to assign the accept() Result<T,E> to a variable
             */
            // let res = listener.accept();

            // if res.is_err() {
            // Continue the iteration despite the error
            // continue;
            // }
            /*
             * 2. Need to unwrap the Result type from accept() by using unwrap()
             * the res.unwrap() will give us s tuple of (TcpStream, SocketAddr)
             */
            // let stream = res.unwrap();

            /*
             * 3. Destructuring the unwrap() value into tuple variable, (stream, addr)
             * and assign each types to the corresponding variables in the tuple.
             *
             * Still not the best way to handle variant from Result as it is only have
             * two variants: Ok(T) or Err(E)
             */
            // let (stream, addr) = res.unwrap();
            // Thus this is where control flow, Match is needed.

            /*
             * Match
             * Allowing to compare a value against a series of patterns
             * and execute the codes matches the pattern
             */
        }
    }
}
