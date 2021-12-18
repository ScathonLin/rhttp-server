use once_cell::sync::Lazy;
use std::{collections::HashMap, env, fs, sync::Mutex};

pub static GLOBAL_MIME_CFG: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let mut entries = HashMap::<String, String>::new();
    let mut exec_dir = env::current_dir().unwrap();
    exec_dir.push("config/mime.json");
    let content_wrapper = fs::read_to_string(exec_dir.to_str().unwrap());
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
    Mutex::new(entries)
});
