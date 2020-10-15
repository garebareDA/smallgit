use super::super::common::serch_dir::SerchDir;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn write_index(dir: SerchDir) -> Result<(), String> {
  //indexステータスをなしでに書き込み
  //まだ前回のコミットと比較できないため
  let index_path = Path::new("./.smallgit/index");
  if !index_path.exists() {
    return Err("index file not found".to_string());
  }
  let mut index_file = File::create(index_path).unwrap();
  for path in dir.get_paths_file().iter() {
    let content = fs::read_to_string(path).unwrap();
    let format_content = format!("blob {}\0{}", content.as_bytes().len(), content);
    let mut hasher = Sha1::new();
    hasher.input_str(&format_content);
    let hex = hasher.result_str();

    index_file
      .write(&format!("{} {}\n", path, hex).as_bytes())
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
        let add_path = Path::new(line_splits[0]);
        if !add_path.exists() {
          return Err("file not found error".to_string());
        }

        let content = fs::read_to_string(add_path).unwrap();
        let format_content = format!("blob {}\0{}", content.as_bytes().len(), content);

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(format_content.as_bytes()).unwrap();
        match e.finish() {
          Ok(byte) => {
            let objects_path_format = format!("{}/{}", objects_path, line_splits[1]);
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
