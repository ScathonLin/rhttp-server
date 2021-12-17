use http::{self, httprequest::HttpRequest};
use std::{env, io::Write, net::TcpListener};
use router::Router;
fn main() {
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.to_str().unwrap();
    println!("current dir is: {}", path);
    let bind_addr = "localhost:9090";
    let server_socket = TcpListener::bind(bind_addr).unwrap();
    println!("rhttp-server started in {}", bind_addr);
    for socket in server_socket.incoming() {
        match socket {
            Ok(mut stream) => {
                let request = HttpRequest::from(&mut stream);
                println!("request is: {:?}", request);
                let resp = Router::route(&request);
                let resp_str: String = resp.into();
                stream.write(resp_str.as_bytes() as &[u8]).unwrap();
            }
            Err(e) => eprintln!(
                "failed to process incoming connection from remote. {:?}",
                e.kind()
            ),
        }
    }
}