use super::super::tree;
use super::super::common::index_readed;
use std::fs::File;

pub struct CommitObject {
  commit_hash:String,
  pub index: Vec<index_readed::IndexReaded>,
  pub tree_dir: Vec<String>,
}

impl CommitObject {
  pub fn new() -> Self {
    Self {
      commit_hash: "".to_string(),
      index: Vec::new(),
      tree_dir: Vec::new(),
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
          self.commit_hash = hash;
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

  pub fn get_hash(self) -> String{
    self.commit_hash
  }
}
