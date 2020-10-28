use super::super::common;
use super::super::common::serch_dir::SerchDir;
use super::super::tree;
use std::fs;
use std::fs::File;
use std::io::Write;
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
    let hex = common::sha1::sha1_gen(&format_content);
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
  match common::index_readed::read_index() {
    Ok(indexs) => {
      for index in indexs {
        let path = &format!("./{}", index.path);
        let add_path = Path::new(path);
        if !add_path.exists() {
          continue;
        }
        let content = fs::read_to_string(add_path).unwrap();
        let format_content = format!("blob {}\0{}", content.as_bytes().len(), content);
        match common::zlib::zlib_encoder(&format_content) {
          Ok(byte) => {
            let objects_path_format = format!("{}/{}", objects_path, index.hex);
            let mut file = File::create(objects_path_format).unwrap();
            file.write_all(&byte).unwrap();
          }
          Err(_) => return Err("zlib encode error".to_string()),
        }
      }
    }

    Err(_) => {
      return Err("There are no modified files".to_string());
    }
  }
  return Ok(());
}
