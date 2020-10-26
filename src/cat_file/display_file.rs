use super::super::common;
use std::fs::File;
use std::io::{Read};
use std::path::Path;

pub fn display(hash:&str) -> Result<(), String> {
  let objects_path = format!("./.smallgit/objects/{}", hash);
  let path = Path::new(&objects_path);
  let mut buffer = Vec::new();
  if !path.exists() {
    return Err("git objects not found".to_string());
  }

  match File::open(path) {
    Ok(mut file) => {
      let _ = file.read_to_end(&mut buffer).unwrap();
      let inner = common::zlib::zlib_dencoder(&buffer);
      let inner_split:Vec<&str> = inner.split("\0").collect();
      let file_type_split: Vec<&str>= inner_split[0].split(" ").collect();
      let file_type = file_type_split[0];
      if file_type == "commit" {
        println!("{}", inner_split[1]);
        return Ok(());
      }

      if file_type == "tree" {
        println!("{}", inner_split[1]);
        return Ok(());
      }

      if file_type == "blob" {
        println!("{}", inner_split[1]);
        return Ok(());
      }

      return Err("git objects error".to_string());
    }

    Err(_) => {
      return Err("file open error".to_string());
    }
  }
}