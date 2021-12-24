use std::{env, fs};

use http::{
    httprequest::{HttpRequest, Resource},
    httpresponse::HttpResponse,
    config::GLOBAL_MIME_CFG,
};

use crate::STATIC_RES;

use super::HttpReqHandler;
use http::config::GLOBAL_STATUSES;

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
        if let Ok(content) = res_content {
            resp.resp_body = Some(content);
        } else {
            let mut path_buf = env::current_dir().unwrap();
            path_buf.push("public/404.html");
            let not_found_page_path = path_buf.to_str().unwrap();
            resp.resp_body = Some(fs::read_to_string(not_found_page_path).unwrap());
            resp.add_header("Content-Type".into(), "text/html".into());
            let statuses = GLOBAL_STATUSES.get().unwrap();
            let status = statuses.get("404").unwrap();
            resp.set_status(status.clone());
            return resp;
        };

        let content_type = if let Some(res_name) = path.split("/").last() {
            match res_name.split(".").last() {
                Some(ext) => GLOBAL_MIME_CFG.get().map(|entries| {
                    if let Some(tp) = entries.get(ext) {
                        tp.clone()
                    } else {
                        "application/octet-stream".into()
                    }
                }),
                None => Some("application/octet-stream".into()),
            }
                .unwrap()
        } else {
            "application/octet-stream".into()
        };
        resp.add_header("Content-Type".into(), content_type);
        resp
    }
}
