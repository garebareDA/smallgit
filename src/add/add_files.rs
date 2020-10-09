use flate2::write::ZlibEncoder;
use flate2::Compression;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct AddFile {
  pub path: String,
  pub paths_dir: Vec<String>,
  pub paths_file: Vec<String>,
  pub ignore: Vec<String>,
}

impl AddFile {
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
      paths_dir: Vec::new(),
      paths_file: Vec::new(),
      ignore: Vec::new(),
    }
  }

  pub fn add_file(&mut self) -> Result<(), String> {
    self.ignore_file();
    match &self.read_dir(&self.path) {
      Ok((dir, file)) => {
        self.set_paths_dir(&dir);
        self.set_paths_file(&file);
      }
      Err(s) => {
        return Err(s.to_string());
      }
    };

    match self.write_index() {
      Ok(_) => {}
      Err(s) => {
        return Err(s);
      }
    }
    //とりあえず objectsに書きだし
    match self.create_objects() {
      Ok(_) => {}
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

  fn write_index(&self) -> Result<(), String> {
    //indexステータスをなしでに書き込み
    //まだ前回のコミットと比較できないため
    let index_path = Path::new("./.smallgit/index");
    if !index_path.exists() {
      return Err("index file not found".to_string());
    }
    let mut index_file = File::create(index_path).unwrap();
    for path in self.get_paths_file().iter() {
      index_file.write(&format!("{}\n", path).as_bytes()).unwrap();
    }
    return Ok(());
  }

  fn create_objects(&self) -> Result<(), String> {
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

          let mut hasher = Sha1::new();
          hasher.input_str(&format_content);
          let hex = hasher.result_str();

          let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
          e.write_all(format_content.as_bytes()).unwrap();
          match e.finish() {
            Ok(byte) => {
              let objects_path_format = format!("{}/{}", objects_path, hex);
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

  pub fn set_paths_dir(&mut self, dir: &Vec<String>) {
    self.paths_dir = dir.clone();
  }

  pub fn set_paths_file(&mut self, file: &Vec<String>) {
    self.paths_file = file.clone();
  }

  pub fn get_paths_file(&self) -> &Vec<String> {
    return &self.paths_file;
  }

  pub fn push_ignore(&mut self, ignore_path: &str) {
    self.ignore.push(ignore_path.to_string());
  }
}
