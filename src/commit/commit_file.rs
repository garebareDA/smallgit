use super::super::tree;
use super::super::tree::tree_git_object::Tree;
use super::index_readed::IndexReaded;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct CommitObject {
  pub index: Vec<IndexReaded>,
  pub tree_dir: Vec<String>,
  pub tree: Tree,
}

impl CommitObject {
  pub fn new() -> Self {
    let tree = Tree::new("/", "");
    Self {
      index: Vec::new(),
      tree_dir: Vec::new(),
      tree,
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
    let mut tree_root = self.generate_tree();
    let mut tree_main = tree::tree_git_object::CommitGet::new();
    match tree_main.tree_main() {
      Ok(_) => {}
      Err(e) => {
        return Err(e);
      }
    }
    self.comparsion_tree(&mut tree_root, &mut tree_main.tree);
    match self.create_tree_file(&mut tree_main.tree) {
      Ok(hash) => match self.create_commit_file(&hash) {
        Ok(()) => {
          self.tree = tree_main.tree;
          self.clear_index();
        }
        Err(e) => {
          return Err(e);
        }
      },
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
          let mut path_format = line_splits[1].to_string();
          path_format.remove(0);
          path_format.remove(0);
          let readed = IndexReaded::new(&path_format, line_splits[2], line_splits[1]);
          self.index.push(readed);
        }
        if self.index.is_empty() {
          return Err("file not staged".to_string());
        }
      }
      Err(_) => {
        return Err("index file not found error".to_string());
      }
    }
    return Ok(());
  }

  fn clear_index(&self) {
    let index_path = "./.smallgit/index";
    File::create(index_path).unwrap();
  }
}
