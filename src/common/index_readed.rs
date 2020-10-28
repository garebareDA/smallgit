use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

#[derive(Clone, Debug)]
pub struct IndexReaded {
  pub path: String,
  pub hex: String,
  pub status: String,
}

impl IndexReaded {
  pub fn new(path: &str, hex: &str, status:&str) -> Self {
    Self {
      path: path.to_string(),
      hex: hex.to_string(),
      status:status.to_string(),
    }
  }
}

pub fn read_index() -> Result<Vec<IndexReaded>, String> {
  let index_path = Path::new("./.smallgit/index");
  let mut index_vec = Vec::new();
  match File::open(index_path) {
    Ok(file) => {
      let reader = BufReader::new(file);
      for line in reader.lines() {
        let line = line.unwrap();
        let line_splits: Vec<&str> = line.split(" ").collect();
        let mut path_format = line_splits[1].to_string();
        path_format.remove(0);
        path_format.remove(0);
        let readed = IndexReaded::new(&path_format, line_splits[2], line_splits[0]);
        index_vec.push(readed);
      }
      if index_vec.is_empty() {
        return Err("file not staged".to_string());
      }
    }
    Err(_) => {
      return Err("index file not found error".to_string());
    }
  }
  return Ok(index_vec);
}