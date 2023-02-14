fn main() {
    /*
     * Create new server instance. Server is struct. new() is associated function and not method.
     * new is called upon Server struct.
     */
    // let server = Server::new("127.0.0.1:8080"); // Intentional error
    // server.run();
}

/*
 * Struct (Data type grouping related data) also refer as class in OOP
 * Define Data with it's type
 */
struct _Server {
    addr: String,
}

/*
 * Add functionality to Server struct, write an implementation block
 */
impl _Server {
    /* Create method or associate function
     *
     * Method in rust always take its first parameter called self
     * self = instance of the struct for the method being called on
     *
     * Associated Function associate with the struct type
     * No need instantiation. Also refer as static function
     * Uppercase Self for @Server struct
     */
    fn _new(addr: String) -> Self {
        // Self or Server in equivalent
        Self { addr }
    }
    /* self pointed to instance of the struct
     * Equivalent to this keyword in other language
     */
    fn _run(self) {}
}
