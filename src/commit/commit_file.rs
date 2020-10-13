use super::index_readed::IndexReaded;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct TakeObject {
  pub file: String,
  pub dir: String,
  pub hex: String,
}

struct TreeObject {
  path_tree:String,
  inner_tree:String
}

pub struct CommitObject {
  index: Vec<IndexReaded>,
  take_object: Vec<TakeObject>,
  tree_object:Vec<TreeObject>
}

impl CommitObject {
  pub fn new() -> Self {
    Self {
      index: Vec::new(),
      take_object: Vec::new(),
      tree_object: Vec::new(),
    }
  }

  pub fn commit_file(&mut self, path: &Vec<String>) -> Result<(), String> {
    match self.read_index() {
      Ok(()) => {}
      Err(s) => {
        return Err(s);
      }
    }
    self.read_object(path);
    self.create_tree();
    println!("{:?}", self.take_object);
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

  fn read_object(&mut self, paths_dirs: &Vec<String>) {
    for paths in self.index.iter() {
      for paths_dir in paths_dirs.iter() {
        let path = &paths.clone().get_path();
        let hex = &paths.clone().get_hex();
        let mut path_dir = paths_dir.to_string();
        path_dir.remove(0);

        let paths_dir_split: Vec<&str> = path_dir.split("/").collect();
        let path_split: Vec<&str> = path.split("/").collect();
        let dir_len = paths_dir_split.len() - 1;
        let path_len = path_split.len() - 2;

        if path_len == 0 {
          let take_object = TakeObject {
            file: path.to_string(),
            dir: "/".to_string(),
            hex: hex.to_string(),
          };
          self.take_object.push(take_object);
          break;
        }

        if dir_len == path_len {
          let take_object = TakeObject {
            file: path.to_string(),
            dir: path_dir.to_string(),
            hex: hex.to_string(),
          };
          self.take_object.push(take_object);
          break;
        }

        for (index, dir) in paths_dir_split.iter().enumerate() {
          if !(dir == &path_split[index]) && !(dir_len == path_len) {
            break;
          }

          if dir_len == index {
            let take_object = TakeObject {
              file: path.to_string(),
              dir: path_dir.to_string(),
              hex: hex.to_string(),
            };
            self.take_object.push(take_object);
          }
          continue;
        }
      }
    }
  }

  fn create_tree(&self) {
    //使うパスをすべて格納
    let mut paths_dir: Vec<&str> = Vec::new();
    for path in self.take_object.iter() {
      paths_dir.push(&path.dir);
    }
    paths_dir.sort();
    paths_dir.dedup();
    println!("{:?}", paths_dir);
  }
}
