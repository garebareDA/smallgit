use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct IndexReaded {
  path: String,
  hex: String,
}

impl IndexReaded {
  pub fn new(path: &str, hex: &str) -> Self {
    Self {
      path: path.to_string(),
      hex: hex.to_string(),
    }
  }

  pub fn get_path(self) -> String {
    self.path
  }

  pub fn get_hex(self) -> String {
    self.hex
  }
}

pub struct CommitObject {
  index: Vec<IndexReaded>,
}

impl CommitObject {

  pub fn new() ->Self {
    Self{
      index: Vec::new(),
    }
  }

  pub fn commit_file(&mut self) {
    self.read_index();
  }

  fn read_index(&mut self) -> Result<(), String> {
    let index_path = Path::new("./.smallgit/index");
    match File::open(index_path) {
      Ok(file) => {
        let reader = BufReader::new(file);
        for line in reader.lines() {
          let line = line.unwrap();
          let line_splits: Vec<&str> = line.split(" ").collect();
          let readed = IndexReaded::new(line_splits[0], line_splits[1]);
          self.index.push(readed);
        }
      }
      Err(_) => {
        return Err("index file not found error".to_string());
      }
    }

    return Ok(());
  }
}
