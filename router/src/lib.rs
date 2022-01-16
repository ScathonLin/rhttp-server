use std::io::Write;
use std::net::TcpStream;
use handler::api::ApiHandler;
use http::{
    httprequest::{HttpRequest, Method, Resource},
    httpresponse::HttpResponse,
};

use crate::handler::{HttpReqHandler, staticres::StaticResHandler};

mod handler;
mod parser;
mod proxy;

const STATIC_RES: &str = "/staticres";

pub struct Router {}

impl Router {
    pub fn route(request: &HttpRequest) -> HttpResponse {
        let mut resp = HttpResponse::default();
        let Resource::Path(path) = &request.resource;
        if path.starts_with(STATIC_RES) {
            let handler_wrapper = get_request_handler(request);
            if let Some(handler) = handler_wrapper {
                resp = handler.handle(request);
                return resp;
            }
        }

        // url API 测试begin
        let proxy_path = parser::parse(&"http://127.0.0.1:8080/user/100?name=linhuadong".into());
        let url_parsed = url::Url::parse(&proxy_path).unwrap();
        let schema = url_parsed.scheme();
        let domain = url_parsed.host_str().unwrap_or("localhost");
        let path = url_parsed.path();
        let query = url_parsed.query().unwrap_or("");
        let port = url_parsed.port().unwrap();
        println!("schema: {}, domain: {}, port: {}, path: {}, query: {}", schema, domain, port, path, query);
        // url API 测试end

        let mut proxy_req = request.clone();
        let proxy_addr = format!("{}:{}", domain, port);
        proxy_req.resource = Resource::Path(String::from(path));
        let mut client = TcpStream::connect(proxy_addr).unwrap();
        let req_str: String = proxy_req.into();
        client.write_all(req_str.as_bytes()).unwrap();
        client.flush().unwrap();
        match request.method {
            Method::GET => {
                resp.set_body("this is api response.".into());
            }
            _ => {
                resp.set_body("hello world".into());
            }
        }
        resp
    }
}

const STATIC_RES_HANDLER: StaticResHandler = StaticResHandler {};
const API_RES_HANDLER: ApiHandler = ApiHandler {};

fn get_request_handler(req: &HttpRequest) -> Option<Box<&dyn HttpReqHandler>> {
    match req.method {
        Method::GET => Some(Box::new(&STATIC_RES_HANDLER)),
        _ => Some(Box::new(&API_RES_HANDLER)),
    }
}
