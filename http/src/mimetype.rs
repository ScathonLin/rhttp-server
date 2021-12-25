use std::collections::HashMap;

use once_cell::sync::OnceCell;

pub static GLOBAL_MIME_CFG: OnceCell<HashMap<String, String>> = OnceCell::new();

pub struct MimeType(pub String, pub String);

pub static OCTECT_STREAM: &'static str = "application/octet-stream";

pub(crate) fn init_default_mime_type() {
    let mut entries = HashMap::<String, String>::new();
    let content_wrapper = utils::read_file_to_string_rel_to_runtime_dir("config/mime.json");
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