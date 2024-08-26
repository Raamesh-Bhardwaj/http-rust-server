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
        println!("Listening to connection requests on {}", self.addr)
    }
}