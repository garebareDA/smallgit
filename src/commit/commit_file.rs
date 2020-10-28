use super::super::tree;
use super::super::tree::tree_git_object::Tree;
use super::super::common::index_readed;
use std::fs::File;

pub struct CommitObject {
  pub index: Vec<index_readed::IndexReaded>,
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
    match index_readed::read_index() {
      Ok(index) => {
        self.index = index;
      }
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

  fn clear_index(&self) {
    let index_path = "./.smallgit/index";
    File::create(index_path).unwrap();
  }
}
