use std::{env, fs};

use http::{
    httprequest::{HttpRequest, Resource},
    httpresponse::HttpResponse,
};

use crate::STATIC_RES;

use super::HttpReqHandler;

#[derive(Default)]
pub struct StaticResHandler {}

impl HttpReqHandler for StaticResHandler {
    fn handle(&self, request: &HttpRequest) -> HttpResponse {
        let mut resp = HttpResponse::default();
        let Resource::Path(ref path) = request.resource;
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
        resp
    }
}
