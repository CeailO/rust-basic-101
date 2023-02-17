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
        let _listener = TcpListener::bind(&self.addr).unwrap();
        /*
         * Concept of while loop
         * while <conditional statement / bool> {}
         */
        // while true {} // infinite loop
        /*
         * Also infinte loop which commonly use: loop {}
         *
         * Unique feature: Loop also can be labeled
         * <apostrophe><label_name>: loop {}
         */
        'outer: loop {
            /*
             * break - Exit the loop
             * continue - Continue the iteration of loop
             */
            loop {
                break 'outer;
            }
        }
    }
}
