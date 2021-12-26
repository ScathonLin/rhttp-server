use std::{
    collections::HashMap,
};

use async_std::{
    io::BufReader,
    net::TcpStream,
};
use futures::{AsyncBufReadExt, AsyncReadExt};


#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    UNKNOWN,
    // 暂时先支持这么多.
}

impl From<&str> for Method {
    fn from(method: &str) -> Self {
        let lowercase_method = method.to_lowercase();
        let res = lowercase_method.as_str();
        match res {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            "delete" => Method::DELETE,
            _ => Method::UNKNOWN,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Version {
    HTTP1_1,
    HTTP2_0,
    UNKNOWN,
}

impl From<&str> for Version {
    fn from(ver: &str) -> Self {
        let ver_in_lowercase = ver.to_lowercase();
        match ver_in_lowercase.as_str() {
            "http/1.1" => Version::HTTP1_1,
            "http/2.0" => Version::HTTP2_0,
            _ => Version::UNKNOWN,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: Option<HashMap<String, String>>,
    pub msg_body: Option<String>,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: Method::GET,
            version: Version::HTTP1_1,
            resource: Resource::Path(String::from("/")),
            headers: None,
            msg_body: None,
        }
    }
}

impl HttpRequest {
    pub async fn from(stream: &mut TcpStream) -> Self {
        let mut reader = BufReader::new(stream);
        let mut request = HttpRequest::default();
        let mut headers = HashMap::<String, String>::new();
        let mut content_len = 0;
        let mut is_req_line = true;
        loop {
            let mut line = String::from("");
            reader.read_line(&mut line).await.unwrap();
            if is_req_line {
                if line.is_empty() && is_req_line {
                    // if the request line is empty, skip and return default HttpRequest;
                    return HttpRequest::default();
                }
                // parse and set http request info.
                let (method, resource, version) = process_req_line(line.as_str());
                request.method = method;
                request.resource = resource;
                request.version = version;
                is_req_line = false;
            } else if line.contains(":") { // process headers;
                let (key, value) = process_request_header(line.as_str());
                headers.insert(key.clone(), value.clone().trim().to_string());
                if key == "Content-Length" {
                    content_len = value.trim().parse::<usize>().unwrap();
                }
            } else if line == String::from("\r\n") {
                // the split line between headers and body;
                break;
            }
        }
        request.headers = Some(headers);
        if content_len > 0 {
            let mut buf = vec![0 as u8; content_len];
            let buf_slice = buf.as_mut_slice();
            // 读取请求体，注意，这里不能在使用stream进行读取，否则会一直卡在这里，要继续用reader进行读取.
            // BufReader::read(&mut reader, buf_slice).await.unwrap();
            reader.read(buf_slice).await.unwrap();
            request.msg_body = Some(String::from_utf8_lossy(buf_slice).to_string());
        }
        request
    }
}

impl From<String> for HttpRequest {
    fn from(diagram: String) -> Self {
        let mut request = HttpRequest::default();
        let mut headers = HashMap::<String, String>::new();
        for line in diagram.lines() {
            if line.contains("HTTP") {
                // process request line.
                println!("request line is: {}", line);
                let (method, resource, version) = process_req_line(line);
                request.method = method;
                request.resource = resource;
                request.version = version;
            } else if line.contains(":") {
                // process request header;
                let (key, value) = process_request_header(line);
                headers.insert(key, value);
            } else if line.is_empty() {
                // skip the line before msg body.
            } else {
                // process msg body;
                request.msg_body = Some(line.to_string());
            }
        }
        request.headers = Some(headers);
        request
    }
}

fn process_request_header(line: &str) -> (String, String) {
    let mut seg_iter = line.split(":");
    (
        seg_iter.next().unwrap().into(),
        seg_iter.next().unwrap().into(),
    )
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut segments = line.split_whitespace();
    (
        segments.next().unwrap().into(),
        Resource::Path(segments.next().unwrap().to_string()),
        segments.next().unwrap().into(),
    )
}

#[cfg(test)]
mod method_testsuite {
    use super::*;

    #[test]
    fn test_method_match() {
        let m: Method = "GET".into();
        assert_eq!(Method::GET, m);
        let m: Method = "posT".into();
        assert_eq!(Method::POST, m);
    }
}

#[cfg(test)]
mod version_testsuite {
    use super::*;

    #[test]
    fn test_version_match() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::HTTP1_1);

        let v: Version = "Http/2.0".into();
        assert_eq!(v, Version::HTTP2_0);

        let v: Version = "HTTP/1.3".into();
        assert_eq!(v, Version::UNKNOWN);
    }
}

#[cfg(test)]
mod http_request_testsuite {
    use super::*;

    #[test]
    fn test_request_parse() {
        let req =
            "GET /index.js HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/html\r\n\r\nxxxx";
        let actual_request: HttpRequest = String::from(req).into();
        let expected_request = HttpRequest {
            method: Method::GET,
            resource: Resource::Path("/index.js".to_string()),
            version: Version::HTTP1_1,
            headers: {
                let mut h = HashMap::<String, String>::new();
                h.insert("Host".to_string(), " localhost".to_string());
                h.insert("Content-Type".to_string(), " text/html".to_string());
                Some(h)
            },
            msg_body: Some(String::from("xxxx")),
        };
        assert_eq!(expected_request, actual_request);
    }
}
