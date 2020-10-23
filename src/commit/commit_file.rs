use super::super::tree;
use super::super::tree::tree_git_object::Tree;
use super::index_readed::IndexReaded;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct CommitObject {
  index: Vec<IndexReaded>,
  tree_dir: Vec<String>,
  tree: Tree,
}

impl CommitObject {
  pub fn new() -> Self {
    let mut vec: Vec<String> = Vec::new();
    vec.push("/".to_string());
    Self {
      index: Vec::new(),
      tree_dir: vec,
      tree: Tree {
        name: "/".to_string(),
        hash: "".to_string(),
        blob: Vec::new(),
        tree: Vec::new(),
      },
    }
  }

  pub fn commit_file(&mut self) -> Result<(), String> {
    match self.read_index() {
      Ok(()) => {}
      Err(s) => {
        return Err(s);
      }
    }
    self.extraction_dir();
    let mut commit = tree::tree_git_object::Commit::new();
    match commit.tree_main() {
      Ok(_) => {}
      Err(e) => {
        return Err(e);
      }
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

  fn extraction_dir(&mut self) {
    for index in self.index.iter() {
      let index_path = index.clone().get_path();
      let mut index_path_split: Vec<&str> = index_path.split("/").collect();
      index_path_split.remove(0);
      index_path_split.remove(index_path_split.len() - 1);

      let connect = index_path_split.connect("/");
      if connect != "" {
        self.tree_dir.push(connect.clone());
      }
      if !index_path_split.is_empty() {
        self.tree_dir.push(index_path_split[0].to_string());
      }

      for path in index_path_split {
        let re = Regex::new(&format!(r"${}{}{},{}", path, "{", path.len(), "}")).unwrap();
        match re.captures(&connect) {
          Some(_) => {
            self.tree_dir.push(path.to_string());
          }
          None => {}
        }
      }
      self.tree_dir.sort();
      self.tree_dir.dedup();
    }
  }
}
