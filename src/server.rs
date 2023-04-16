use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection established on {}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) =>{ 
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer [..]){
                                Ok(_) => println!("success"),
                                Err(error) => println!("Error: {}", error)
                            }
                        },
                        Err(error) => println!("Error, {} ", error)
                    }
                },
                Err(error) => {
                    println!("failed to establish connection {}", error)
                }
            }
        }
    }
}
 