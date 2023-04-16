
use http::request::Request;
use http::method::Method;
use server::Server;

mod server;
mod http;
fn main() {
    let get: Method = Method::GET;
    let request: Request = Request::new("this is path".to_string(), Some("this is query".to_string()), get);

    let ip: String = String::from("127.0.0.1:8080".to_string());
    let server: Server = Server::new(ip);
    server.run();
}

