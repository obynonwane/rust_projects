use std::io::Read; // import the Read trait from the std::io module to read data from the stream
use std::net::TcpListener; // import the TcpListener struct from the std::net module

// define a struct called Server with a single field addr of type String
pub struct Server {
    addr: String,
}

// implement methods for the Server struct
impl Server {
    // constructor for Server struct
    pub fn new(addr: String) -> Self {
        // create a new Server instance with the given address
        Self {
            addr: addr.to_string(),
        }
    }

    // method accept self
    pub fn run(self) {
        println!("Server running on {}", &self.addr);

        // create a TcpListener that binds to the address specified in the Server struct
        let listener = TcpListener::bind(&self.addr).unwrap();

        // infinit loop to keep the server running
        loop {
            // listen for incoming connections
            match listener.accept() {
                // if a connection is accepted, print the address of the client
                Ok((mut stream, addr)) => {
                    println!("New connection from {}", addr);

                    // declare a buffer to read data from the stream
                    let mut buffer = [0; 1024];

                    // read data from the stream into the buffer
                    match stream.read(&mut buffer) {
                        // if the read is successful, print the number of bytes read and the data
                        Ok(bytes_read) => {
                            println!("Received {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);
                        }

                        // if there is an error reading from the stream, print the error
                        Err(e) => {
                            eprintln!("Error reading from stream: {}", e);
                        }
                    }
                    // handle stream here
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                    continue;
                }
            }
        }
    }
}
