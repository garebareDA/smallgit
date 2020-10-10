use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs;

pub struct SerchDir {
  path: String,
  paths_dir: Vec<String>,
  paths_file: Vec<String>,
  ignore: Vec<String>,
}

impl SerchDir {
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
      paths_dir: Vec::new(),
      paths_file: Vec::new(),
      ignore: Vec::new(),
    }
  }

  pub fn serch_dir(&mut self) -> Result<(), String>{
    self.ignore_file();
     match self.read_dir(&self.path) {
      Ok((dir, file)) => {
        self.set_paths_dir(&dir);
        self.set_paths_file(&file);
      }

      Err(s) => {
        return Err(s);
      }
    }
    return Ok(());
  }

  fn read_dir(&self, path: &str) -> Result<(Vec<String>, Vec<String>), String> {
    let mut path_dir: Vec<String> = Vec::new();
    let mut path_file: Vec<String> = Vec::new();
    let path_is = Path::new(path);
    if path_is.is_file() {
      path_file.push(path.to_string());
      return Ok((path_dir, path_file));
    }

    if !path_is.exists() {
      return Err(format!("not foud {}", path));
    }

    let file = fs::read_dir(path).unwrap();
    for path in file {
      let path_in = path.unwrap();
      let paths = path_in.path().display().to_string();
      if self.is_gitignore(&paths) || paths == "./.smallgit" || paths == "./.git" {
        continue;
      }

      if path_in.file_type().unwrap().is_dir() {
        path_dir.push(paths.to_string());
        let (mut dir, mut file) = self.read_dir(&paths).unwrap();
        if !dir.is_empty() {
          path_dir.append(&mut dir);
        };

        if !file.is_empty() {
          path_file.append(&mut file);
        }
      } else {
        path_file.push(paths.to_string())
      }
    }
    return Ok((path_dir, path_file));
  }

  fn ignore_file(&mut self) {
    let git_ignore_path = "./.gitignore";
    match File::open(git_ignore_path) {
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

  //相対パスのみ
  fn is_gitignore(&self, path: &str) -> bool {
    let mut path = path.to_string();
    path.remove(0);
    let mut path: Vec<&str> = path.split("/").collect();
    path.retain(|x| x != &"");
    for ignore in self.ignore.iter() {
      let mut ignore_sprits: Vec<&str> = ignore.split("/").collect();
      ignore_sprits.retain(|x| x != &"");
      let ignore_len = ignore_sprits.len() - 1;
      'inner: for (index, ignore_sprlit) in ignore_sprits.iter().enumerate() {
        let path_split = path[index];
        if ignore_sprlit == &path_split {
          if ignore_len == index {
            return true;
          }
          continue 'inner;
        } else {
          break 'inner;
        }
      }
    }
    return false;
  }

  pub fn set_paths_dir(&mut self, dir: &Vec<String>) {
    self.paths_dir = dir.clone();
  }

  pub fn set_paths_file(&mut self, file: &Vec<String>) {
    self.paths_file = file.clone();
  }

  pub fn get_paths_file(&self) -> &Vec<String> {
    return &self.paths_file;
  }

  pub fn get_paths_dir(&self) -> &Vec<String> {
    return &self.paths_dir;
  }

  pub fn push_ignore(&mut self, ignore_path: &str) {
    self.ignore.push(ignore_path.to_string());
  }
}