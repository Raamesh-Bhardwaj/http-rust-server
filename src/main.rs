use server::Server;
// use http;

mod server;

fn main() {
    let server: Server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod http {
    pub mod request{
        use super::methods::Method;

        pub struct Request {
            path: String,
            query_params: Option<String>,
            method: Method
        }
    }

    pub mod methods {

        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }


    }
}