use std::{io::Write, net::TcpListener};

use http::{self, config, httprequest::HttpRequest};
use router::Router;

fn main() {
    config::init();
    println!("finished to init config");
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