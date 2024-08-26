use std::net::TcpListener;

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
    }
}