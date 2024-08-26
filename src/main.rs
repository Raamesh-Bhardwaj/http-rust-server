use server::Server;
use http_local::Request;

mod server;
mod http_local;

fn main() {
    let server: Server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}