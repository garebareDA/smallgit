use super::super::tree;
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
}
