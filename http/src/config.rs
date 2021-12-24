use serde_json::Result as SerdResult;
use std::{
    collections::HashMap,
    env, fs,
    io::Result as IoResult,
};
use once_cell::sync::OnceCell;

pub static GLOBAL_MIME_CFG: OnceCell<HashMap<String, String>> = OnceCell::new();

fn read_file_to_string_rel_to_runtime_dir(file_path: &str) -> IoResult<String> {
    let mut runtime_dir = env::current_dir().unwrap();
    runtime_dir.push(file_path);
    return fs::read_to_string(runtime_dir.to_str().unwrap());
}

pub fn init() {
    init_default_mime_type();
    init_default_http_status();
}

fn init_default_mime_type() {
    let mut entries = HashMap::<String, String>::new();
    let content_wrapper = read_file_to_string_rel_to_runtime_dir("config/mime.json");
    if let Ok(mime_cfg_json) = content_wrapper {
        let json: HashMap<String, String> = serde_json::from_str(mime_cfg_json.as_str()).unwrap();
        for (k, v) in json.iter() {
            v.split_whitespace()
                .filter(|&tp| !tp.is_empty())
                .for_each(|tp| {
                    entries.insert(tp.to_string(), k.to_string());
                })
        }
    }
    println!("begin loading mime config");
    GLOBAL_MIME_CFG.set(entries).unwrap();
}

fn init_default_http_status() {
    let mut status_config = HashMap::<String, Status>::new();
    if let Ok(status_cfg_json) = read_file_to_string_rel_to_runtime_dir("config/status.json") {
        let kvs: SerdResult<HashMap<String, String>> =
            serde_json::from_str(status_cfg_json.as_str());
        match kvs {
            Ok(kvs) => {
                kvs.iter().for_each(|(k, v)| {
                    status_config.insert(k.clone(), Status(k.clone(), v.clone()));
                });
            }
            Err(_) => {
                eprintln!("failed to load status.json");
            }
        }
    }
    GLOBAL_STATUSES.set(status_config).unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub struct Status(pub String, pub String);

impl Status {
}

pub static GLOBAL_STATUSES: OnceCell<HashMap<String, Status>> = OnceCell::new();