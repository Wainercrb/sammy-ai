use std::fs;
use crate::shared::errors::FILE_NOT_FOUND;


pub fn list_files_in_directory(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .expect(FILE_NOT_FOUND)
        .filter_map(Result::ok)
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
<<<<<<< HEAD
=======
}

enum Actions {
    Create,
    Read(String)
}

pub fn testing() {
   let act: Actions = Actions::Read("abc".to_string());

   match act {
       Actions::Create => {
           println!("Action is Create");
       },
       Actions::Read(file) => {
           println!("Action is Read with file: {}", file);
       },

   }
>>>>>>> bcb9c5a (feat: add shared module for file system operations and error handling)
}