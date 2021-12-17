use http::{
    httprequest::{HttpRequest, Method, Resource},
    httpresponse::HttpResponse,
};
use std::{env, fs};

const STATIC_RES: &str = "/staticres";

pub struct Router {}

impl Router {
    pub fn route(request: &HttpRequest) -> HttpResponse {
        let mut resp = HttpResponse::default();
        match request.method {
            Method::GET => {
                let Resource::Path(path) = &request.resource;
                if path.starts_with(STATIC_RES) {
                    // visit static resource.
                    let real_path = &path[STATIC_RES.len()..];
                    let mut runtime_dir = env::current_dir().unwrap();
                    runtime_dir.push("public");
                    real_path
                        .split("/")
                        .into_iter()
                        .for_each(|seg| runtime_dir.push(seg));
                    let res_content = fs::read_to_string(runtime_dir.to_str().unwrap());
                    resp.resp_body = res_content.ok();
                    resp.add_header("Content-Type", "text/html");
                }
            }
            _ => {}
        }
        resp
    }
}
