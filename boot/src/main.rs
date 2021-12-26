use async_std::net::TcpListener;
use futures::{AsyncWriteExt, StreamExt};

use http::{self, config, httprequest::HttpRequest};
use router::Router;

#[async_std::main]
async fn main() {
    config::init();
    println!("finished to init config");
    let bind_addr = "localhost:9090";
    let server_socket = TcpListener::bind(bind_addr).await.unwrap();
    server_socket.incoming().for_each_concurrent(None, |tcpstream| async move {
        match tcpstream {
            Ok(mut stream) => {
                let request = HttpRequest::from(&mut stream).await;
                println!("request is: {:?}", request.resource);
                let resp = Router::route(&request).await;
                let resp_str: String = resp.into();
                stream.write(resp_str.as_bytes() as &[u8]).await.unwrap();
                stream.flush().await.unwrap();
            }
            Err(e) => eprintln!(
                "failed to process incoming connection from remote. {:?}",
                e.kind()
            ),
        };
    }).await;
    println!("rhttp-server started in {}", bind_addr);
}