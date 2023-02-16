use std::net::TcpListener;
// No need to use std::result::Result; because imported by default.

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
         * bind() (associate function) doesn't return TcpListener, instead it return TcpListener that wrapped in Result.
         *
         * There's no concept of exception in rust. As such, it's constitution from Result
         * This concept differ from language that have garbage collectors.
         *
         * One need to acknowledge the possibility that the program will crash/ error occurs ahead of compilation
         * and take countermeausure before code is compiled.
         *
         * Errors category
         * recoverable - example: file not found
         * unrecoverable - accessing index that beyond the bound of array end.
         *
         * T - Type
         * E - Error
         *
         * Explaination based on result.rs:
         * Result<T, E> enum can be either Ok(T) or Err(E)
         * unwrap() is called on Result<T, E> from bind() function
         * Result return Ok(T) (in this case TcpListener)
         * Thus unwrap() unwrapping Ok(TcpListener) to obtain TcpListener
         * If Result return Err(E), the program terminated
         * 
         * Further walkthrough available using Windows OS.
         * Netcat(ncat) is require upon this test, the package can be obtained from Nmap installation
         * run `ncat -k -l 127.0.0.1 8080` on another bash to run ncat on localhost with port 8080
         * 
         * Running the rust program, the rust program will terminated as it tried to listen on localhost with port 8080
         * Subsequent reason will be given from compiler error message.
         */
        let _listener = TcpListener::bind(&self.addr).unwrap();
    }
}
