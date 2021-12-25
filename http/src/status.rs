use once_cell::sync::OnceCell;
use std::collections::HashMap;
use serde_json::Result;

pub(crate) fn init_default_http_status() {
    let mut status_config = HashMap::<String, Status>::new();
    if let Ok(status_cfg_json) = utils::read_file_to_string_rel_to_runtime_dir("config/status.json") {
        let kvs: Result<HashMap<String, String>> =
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

impl Status {}

pub static GLOBAL_STATUSES: OnceCell<HashMap<String, Status>> = OnceCell::new();