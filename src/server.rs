use std::{net::{TcpListener, TcpStream}, os::unix::net::SocketAddr};
use std::io::Read;
use crate::http_local::Request;
use std::convert::TryFrom;
use std::convert::TryInto;


pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self{
        return Self {
            addr: addr
        }
    }

    pub fn run(self){
        println!("Listening to connection requests on {}", self.addr);

        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved Request as {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) =>{

                                },
                                Err(e) => println!("Failed to parse Error: {}", e)
                            }

                        }, 
                        Err(e) => {
                            println!("Error Occured in reading the request to buffer: {}", e);
                        }
                    }

                },
                Err(e) => {
                    println!("Stream Establishment failed with Error: {}", e)
                }
                
            }
        }
    }
}