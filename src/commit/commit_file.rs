use super::index_readed::IndexReaded;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct TakeObject {
  pub file: String,
  pub dir: String,
  pub hex: String,
}

#[derive(Clone, Debug)]
struct TreeObject {
  pub path_tree: String,
  pub inner: String,
  pub hex: String,
}

pub struct CommitObject {
  index: Vec<IndexReaded>,
  take_object: Vec<TakeObject>,
  tree_object: Vec<TreeObject>,
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
    match self.create_tree() {
      Ok(()) => {}
      Err(e) => {
        return Err(e);
      }
    }
    match self.create_tree_file() {
      Ok(()) => {}
      Err(e) => return Err(e),
    }
    match self.create_commit_file() {
      Ok(()) => {}
      Err(e) => return Err(e),
    }
    match self.clear_index() {
      Ok(()) => {}
      Err(e) => return Err(e),
    }
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

        for (index, dir) in paths_dir_split.iter().enumerate() {
          if dir == &path_split[index] && dir_len == path_len && dir_len == index {
            let take_object = TakeObject {
              file: path.to_string(),
              dir: path_dir.to_string(),
              hex: hex.to_string(),
            };
            self.take_object.push(take_object);
            break;
          }
          continue;
        }
      }
    }
  }

  fn create_tree(&mut self) -> Result<(), String> {
    //使うパスをすべて格納
    //先頭にマッチできるようになる
    //スプリットして要素数が一つの場合ルートに付属
    let mut paths_dir: Vec<&str> = Vec::new();
    for path in self.take_object.iter() {
      paths_dir.push(&path.dir);
    }
    paths_dir.sort();
    paths_dir.dedup();
    for paths in paths_dir.iter() {
      let tree_object = TreeObject {
        path_tree: paths.to_string(),
        inner: "\0".to_string(),
        hex: "".to_string(),
      };
      self.tree_object.push(tree_object);
    }

    for object in self.take_object.iter() {
      let dir = &object.dir;
      let file = &object.file;
      let hex = &object.hex;

      for index in 0..self.tree_object.len() {
        let paths_tree = &self.tree_object[index].path_tree;
        if paths_tree == dir {
          let inner = &self.tree_object[index].inner;
          let file: Vec<&str> = file.split("/").collect();
          let file_name = file[file.len() - 1];
          self.tree_object[index].inner = format!("{}brob {} {}\n", inner, file_name, hex);
        }
      }
    }

    for index in (0..self.tree_object.len()).rev() {
      let inner = &self.tree_object[index].inner;
      let format_inner = format!("tree {}{}", inner.to_string().as_bytes().len(), inner);
      let mut hasher = Sha1::new();
      hasher.input_str(&format_inner);
      let hex = hasher.result_str();
      self.tree_object[index].inner = format_inner;
      self.tree_object[index].hex = hex;
    }

    for index in (0..self.tree_object.len()).rev() {
      let path = self.tree_object[index].path_tree.to_string();
      for inner_index in 0..self.tree_object.len() {
        let tree_path = self.tree_object[inner_index].path_tree.to_string();
        let tree_path_split: Vec<&str> = tree_path.split("/").collect();
        if path == tree_path {
          continue;
        }

        if path == "/" {
          if tree_path_split.len() == 2 {
            self.change_tree_object(index, inner_index, &tree_path);
          }
          continue;
        }

        let reg = Regex::new(&format!(r"^{}", path)).unwrap();
        match reg.captures(&tree_path) {
          Some(_) => {}
          None => {
            continue;
          }
        }

        self.change_tree_object(index, inner_index, &tree_path);
      }
    }
    return Ok(());
  }

  fn create_tree_file(&self) -> Result<(), String> {
    let objects_path = "./.smallgit/objects";
    if !Path::new(objects_path).exists() {
      return Err("objects dir is not found".to_string());
    }
    for tree in self.tree_object.iter() {
      let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
      e.write_all(tree.inner.as_bytes()).unwrap();
      match e.finish() {
        Ok(byte) => {
          let objects_path_format = format!("{}/{}", objects_path, tree.hex);
          let mut file = File::create(objects_path_format).unwrap();
          file.write_all(&byte).unwrap();
        }
        Err(_) => return Err("zlib encode error".to_string()),
      }
    }
    return Ok(());
  }

  fn create_commit_file(&self) -> Result<(), String> {
    let objects_path = "./.smallgit/objects";
    if !Path::new(objects_path).exists() {
      return Err("objects dir is not found".to_string());
    }
    for tree in self.tree_object.iter() {
      if tree.path_tree == "/" {
        let inner = format!("tree {}\n", tree.hex);
        let commit = format!("commit {}\0{}", inner.as_bytes().len(), inner);
        let mut hasher = Sha1::new();
        hasher.input_str(&commit);
        let hex = hasher.result_str();
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(commit.as_bytes()).unwrap();
        match e.finish() {
          Ok(byte) => {
            let objects_path_format = format!("{}/{}", objects_path, hex);
            let mut file = File::create(objects_path_format).unwrap();
            file.write_all(&byte).unwrap();
            match self.wirite_ref_main(&hex) {
              Ok(_) => {}
              Err(e) => {
                return Err(e);
              }
            }
          }
          Err(_) => return Err("zlib encode error".to_string()),
        }
        break;
      }
    }
    return Ok(());
  }

  fn wirite_ref_main(&self, hex: &str) -> Result<(), String> {
    let main = "./.smallgit/refs/main";
    if !Path::new(main).exists() {
      return Err("objects dir is not found".to_string());
    }
    let mut file = File::create(main).unwrap();
    match file.write_all(hex.as_bytes()) {
      Ok(_) => {}
      Err(_) => {
        return Err("main brach can't write".to_string());
      }
    }
    return Ok(());
  }

  fn clear_index(&self) -> Result<(), String> {
    let index = "./.smallgit/index";
    if !Path::new(index).exists() {
      return Err("objects dir is not found".to_string());
    }
    File::create(index).unwrap();
    return Ok(());
  }

  fn change_tree_object(&mut self, index: usize, inner_index: usize, tree_path: &str) {
    let tree_path_split: Vec<&str> = tree_path.split("/").collect();
    let hex = &self.tree_object[inner_index].hex;
    let inner = &self.tree_object[index].inner;
    let format_inner = format!(
      "{}tree {} {}\n",
      inner,
      tree_path_split[tree_path_split.len() - 1],
      hex
    );
    let mut hasher = Sha1::new();
    hasher.input_str(&format_inner);
    let hex = hasher.result_str();
    self.tree_object[index].inner = format_inner;
    self.tree_object[index].hex = hex;
  }
}
