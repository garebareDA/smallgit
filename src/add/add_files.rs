use super::super::common;
use super::super::common::serch_dir::SerchDir;
use super::super::tree;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn write_index(dir: SerchDir) -> Result<(), String> {
  //indexステータスをなしでに書き込み
  let index_path = Path::new("./.smallgit/index");
  if !index_path.exists() {
    return Err("index file not found".to_string());
  }
  let mut index_file = File::create(index_path).unwrap();

  let mut tree = tree::tree_git_object::CommitGet::new();
  match tree.tree_main() {
    Ok(_) => {}
    Err(e) => {
      return Err(e);
    }
  }

  for path in dir.get_paths_file().iter() {
    let content = fs::read_to_string(path).unwrap();
    let format_content = format!("blob {}\0{}", content.as_bytes().len(), content);
    let mut hasher = Sha1::new();
    hasher.input_str(&format_content);
    let hex = hasher.result_str();
    let (check, status) = tree.check_blob(path, &hex);
    if !check {
      index_file
        .write(&format!("{} {} {}\n", status, path, hex).as_bytes())
        .unwrap();
    }
  }

  let remove_file = tree.check_remove_blob(&dir);
  if remove_file.is_empty() {
    return Ok(());
  }

  for remove in remove_file.iter() {
    index_file
      .write(&format!("remove {} {}\n", remove.name, remove.hash).as_bytes())
      .unwrap();
  }

  return Ok(());
}

pub fn create_objects() -> Result<(), String> {
  let objects_path = "./.smallgit/objects";
  let index_path = Path::new("./.smallgit/index");
  if !Path::new(objects_path).exists() {
    return Err("objects dir is not found".to_string());
  }

  match File::open(index_path) {
    Ok(file) => {
      let reader = BufReader::new(file);
      for line in reader.lines() {
        let line = line.unwrap();
        let line_splits: Vec<&str> = line.split(" ").collect();
        let add_path = Path::new(line_splits[1]);
        if !add_path.exists() {
          return Err("file not found error".to_string());
        }

        let content = fs::read_to_string(add_path).unwrap();
        let format_content = format!("blob {}\0{}", content.as_bytes().len(), content);
        match common::zlib::zlib_encoder(&format_content) {
          Ok(byte) => {
            let objects_path_format = format!("{}/{}", objects_path, line_splits[2]);
            let mut file = File::create(objects_path_format).unwrap();
            file.write_all(&byte).unwrap();
          }
          Err(_) => return Err("zlib encode error".to_string()),
        }
      }
    }

    Err(_) => {
      return Err("index file is not found".to_string());
    }
  }
  return Ok(());
}
