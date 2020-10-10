use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
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
  pub fn new() -> Self {
    Self { index: Vec::new() }
  }

  pub fn commit_file(&mut self, path:&Vec<String>) -> Result<(), String> {
    match self.read_index() {
      Ok(()) => {}
      Err(s) => {
        return Err(s);
      }
    }

    self.create_tree(path);
    println!("{:?}", self.index);

    return Ok(());
  }

  fn read_index(&mut self) -> Result<(), String> {
    let index_path = Path::new("./.smallgit/index");
    match File::open(index_path) {
      Ok(file) => {
        let reader = BufReader::new(file);
        for line in reader.lines() {
          let line = line.unwrap();
          let line_splits: Vec<&str> = line.split(" ").collect();
          let mut path_format = line_splits[0].to_string();
          path_format.remove(0);
          let readed = IndexReaded::new(&path_format, line_splits[1]);
          self.index.push(readed);
        }
      }
      Err(_) => {
        return Err("index file not found error".to_string());
      }
    }
    return Ok(());
  }

  fn create_tree(&self, path: &Vec<String>) {
    let objects_path = Path::new("./.smallgit/objects");
    
  }
}
