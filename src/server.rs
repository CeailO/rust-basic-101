use std::{
    /*
     * Read() are in form of traits
     * Read traits have no body
     *
     * Act like an interface
     */
    io::Read, net::TcpListener,
};

pub struct Server {
    addr: String,
}

/*
 * Writing definition for function that accepts array from it's parameter,
 * defining size of array is needed for to be known at compilation time.
 * No arbitary size number for array
 */
// fn arr(a: [u8; 5]) {}
/**
 * Slice (Reference/ pointer to the array) that needed to be accepeted
 * The array slice have more in common with string slice except array slice
 * is more generic and can be used for any type (u8 in this example)
 */
// fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                /*
                 *
                 * read fn take &mut self and buf: &mut [u8]
                 * read() take mutable reference to self thus stream variable needed to be mutable
                 */
                Ok((mut stream, _addr)) =>
                /*
                 * Need to read the byte sent by the client
                 *
                 * read() is separated from TcpStream implementation
                 * Arrays defined as [<array_element1>,<array_element2>,...] also known as compound types
                 * Differs to tuple it contain a group of same types
                 */
                {
                    // New array a instance: [i32; 4] where i32 is array type and 4 is array size
                    // let a = [1, 2, 3, 4, 5, 6, 7, 8];
                    /*
                     * Mismatched as arr() expects 5 elements while a having 4 elements
                     * To fix, arr() definition needed to be changed.
                     */
                    // arr(a);

                    // Stopping a pointer within the array a
                    // arr(&a[1..4]);

                    /*
                     * Needed buffer variable in form of array
                     *
                     * Special syntax for creating an array contain same value for each elements
                     *
                     * Contrast from C array initialization, the array will contain random memory
                     * if doesn't deallocate properly from previous memory address access,
                     * and lead to memory corruption
                     *
                     * Rust array initialization:
                     * 1. Allocate chunk of memories enough for size allocation
                     * 2. Re-traverse whole memory region/ allocated array with memories
                     * 3. flip all bits to value provided (in this case is we give 0)
                     */
                    let mut buffer = [0; 1024]; // worth of 1kb allocation

                    // read() will read all the memory from the given array: buffer
                    stream.read(&mut buffer);
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
