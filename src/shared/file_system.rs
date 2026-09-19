use std::fs;
use crate::shared::errors::FILE_NOT_FOUND;


pub fn list_files_in_directory(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .expect(FILE_NOT_FOUND)
        .filter_map(Result::ok)
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}