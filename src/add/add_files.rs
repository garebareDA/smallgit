use std::fs;
use std::io::{BufRead, BufReader};

pub struct AddFile {
  pub path: String,
  pub paths: Vec<String>,
  pub ignore: Vec<String>,
}

impl AddFile {
  pub fn new(path:&str) -> Self {
    Self {
      path:path.to_string(),
      paths: Vec::new(),
      ignore: Vec::new(),
    }
  }

  pub fn add_file(&mut self) {
    self.ignore_file();
    println!("{:?}",self.ignore);
    let dir_vec = self.read_dir(&self.path);
  }

  fn read_dir(&self, path: &str) -> Vec<String> {
    let file = fs::read_dir(path).unwrap();
    let mut path_vec: Vec<String> = Vec::new();
    for path in file {
      let path_in = path.unwrap();
      let paths = path_in.path().display().to_string();
      path_vec.push(paths.to_string());

      if path_in.file_type().unwrap().is_dir() {
        let mut paths_vec = self.read_dir(&paths);
        if !paths_vec.is_empty() {
          path_vec.append(&mut paths_vec)
        };
      }
    }

    return path_vec;
  }

  fn ignore_file(&mut self) {
    let git_ignore_path = "./.gitignore";
    match fs::File::open(git_ignore_path) {
      Ok(file) => {
        let reader = BufReader::new(file);
        for line in reader.lines() {
          let line = line.unwrap();
          self.push_ignore(&line);
        }
      }
      Err(_) => {
        return;
      }
    }
  }

  pub fn get_paths(&self) -> &Vec<String> {
    return &self.paths;
  }

  pub fn get_ignore(&self) -> &Vec<String> {
    return &self.ignore;
  }

  pub fn set_paths(&mut self, paths: &Vec<String>) {
    self.paths = paths.clone();
  }

  pub fn set_ignore(&mut self, ignore: &Vec<String>) {
    self.paths = ignore.clone();
  }

  pub fn push_ignore(&mut self, ignore_path:&str) {
    self.ignore.push(ignore_path.to_string());
  }
}
