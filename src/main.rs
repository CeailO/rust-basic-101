fn main() {
    /*
     * Normal String can be expanded and shrink dynamically at compile time
     * &str (String slice) is immutable because was specified at compile time
     */
    // let string = String::from("127.0.0.1:8080"); // length of string is 14
    // let string_slice = &string[10..]; // Take the last 4 digits by specifying ranges interest.
    // let string_borrow: &str = &string; // Borrow the string and convert into string slice.

    // let string_literal = "1234"; // The size of the string slice is known at compile time. Thus it baked into binary itself.

    // Cyrillic characters have 2 bytes while most of symbol have 4 bytes.
    // let string_2 = String::from("ѝ🍟🍺🍺");

    // let string_slice_2 = &string_2[..8]; // Take the first 8 bytes.
    // let string_borrow_2: &str = &string_2;

    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);

    // dbg!(string_literal);

    // dbg!(&string_2);
    // Error because a complete string slices would be 10 until the first beer symbol
    // dbg!(string_slice_2);
    // dbg!(string_borrow_2);
    let server = Server::new("127.0.0.1:8080".to_string()); // Intentional error
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }
    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
