use std::fs;
use std::fs::File;
use std::io::Read;

use super::super::common::zlib;

#[derive(Clone, Debug)]
pub struct Tree {
  pub name: String,
  pub hash: String,
  pub blob: Vec<Blob>,
  pub tree: Vec<Tree>,
  pub is_edit:bool,
}

#[derive(Clone, Debug)]
pub struct Blob {
  pub name: String,
  pub hash: String,
}

#[derive(Clone, Debug)]
pub struct CommitGet {
  pub hash: String,
  pub tree: Tree,
}

impl Tree {
  pub fn new(name: &str, hash: &str) -> Self {
    Self {
      name: name.to_string(),
      hash: hash.to_string(),
      blob: Vec::new(),
      tree: Vec::new(),
      is_edit:false,
    }
  }
}

impl Blob {
  pub fn new(name: &str, hash: &str) -> Self {
    Self {
      name: name.to_string(),
      hash: hash.to_string(),
    }
  }
}

impl CommitGet {
  pub fn new() -> Self {
    Self {
      hash: "".to_string(),
      tree: Tree {
        name: String::new(),
        hash: String::new(),
        blob: Vec::new(),
        tree: Vec::new(),
        is_edit:false,
      },
    }
  }

  pub fn set_hasj(&mut self, hash: &str) {
    self.hash = hash.to_string();
  }

  pub fn tree_main(&mut self) -> Result<bool, String> {
    match self.get_refs_main() {
      Ok(main_ref) => {
        if main_ref != "" {
          self.hash = main_ref;
        }else {
          return Ok(false);
        }
      }
      Err(_) => {
        return Err("main branch is abnormal".to_string());
      }
    }

    match self.get_commit_object(&self.hash) {
      Ok(tree_hash) => {
        self.tree.name = "/".to_string();
        self.tree.hash = tree_hash;
      }
      Err(e) => return Err(e),
    }

    match self.tree_go_back() {
      Ok(_) => {}
      Err(e) => {
        return Err(e);
      }
    }
    return Ok(true);
  }

  fn get_refs_main(&self) -> Result<String, String> {
    let main_branch_paht = "./.smallgit/refs/main";
    let main_commit = fs::read_to_string(&main_branch_paht);
    match main_commit {
      Ok(main_ref) => {
        return Ok(main_ref);
      }
      Err(_) => {
        return Err("main branch is abnormal".to_string());
      }
    }
  }

  // commit オブジェクトの取得 中のtreeから遡る
  fn get_commit_object(&self, hash: &str) -> Result<String, String> {
    let commit_tree_path = &format!("./.smallgit/objects/{}", hash);
    let file = File::open(commit_tree_path);
    let mut buffer = Vec::new();
    match file {
      Ok(mut file) => {
        match file.read_to_end(&mut buffer) {
          Ok(_) => {},
          Err(_) => {
            return Err(format!("not found git objects {}", hash));
          }
        }
        let decoded = zlib::zlib_dencoder(&buffer);
        let decoded_split: Vec<&str> = decoded.split("\0").collect();
        let tmp_split: Vec<&str> = decoded_split[0].split("\n").collect();
        let tree_split: Vec<&str> = tmp_split[0].split(" ").collect();
        let tree_hash = tree_split[1].to_string();
        return Ok(tree_hash);
      }
      Err(_) => {
        return Err("commit objects not found".to_string());
      }
    }
  }
}
