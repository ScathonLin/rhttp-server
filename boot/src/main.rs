use std::io::Write;
use std::net::{TcpListener, TcpStream};

use http::{self, config, httprequest::HttpRequest};
use router::Router;
use core as self_core;
use self_core::thread::ThreadPool;


fn main() {
    config::init();
    println!("finished to init config");
    let bind_addr = "localhost:9090";
    let server_socket = TcpListener::bind(bind_addr).unwrap();
    // default 10 threads in thread pool;
    let pool = ThreadPool::new(10);

    for conn_wrapper in server_socket.incoming() {
        match conn_wrapper {
            Ok(stream) => {
                pool.execute(Box::new(|| {
                    handle_connection(stream);
                }))
            }
            Err(e) => eprintln!(
                "failed to process incoming connection from remote. {:?}",
                e.kind()
            ),
        };
    }
    println!("rhttp-server started in {}", bind_addr);
}

fn handle_connection(mut stream: TcpStream) {
    let request = HttpRequest::from(&mut stream);
    println!("request is: {:?}", request.resource);
    let resp = Router::route(&request);
    let resp_str: String = resp.into();
    stream.write(resp_str.as_bytes() as &[u8]).unwrap();
    stream.flush().unwrap();
}