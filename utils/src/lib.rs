use std::{env, fs};
use std::io::Result;

pub fn read_file_to_string_rel_to_runtime_dir(file_path: &str) -> Result<String> {
    let mut runtime_dir = env::current_dir().unwrap();
    runtime_dir.push(file_path);
    return fs::read_to_string(runtime_dir.to_str().unwrap());
}