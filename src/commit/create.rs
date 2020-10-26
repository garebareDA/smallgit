use super::super::common;
use super::super::tree::tree_git_object::Tree;
use super::commit_file;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::File;
use std::io::Write;

impl commit_file::CommitObject {
  pub fn create_tree_file(&mut self, tree: &mut Tree) -> Result<String, String> {
    match self.tree_write(tree) {
      Ok(hash) => {
        return Ok(hash.to_string());
      }

      Err(e) => {
        return Err(e);
      }
    }
  }

  fn tree_write(&mut self, tree: &mut Tree) -> Result<String, String> {
    if tree.is_edit {
      let mut inner = "".to_string();
      for index in 0..tree.tree.len() {
        match self.tree_write(&mut tree.tree[index]) {
          Ok(hash) => {
            tree.tree[index].hash = hash.to_string();
            let path_split:Vec<&str> = tree.tree[index].name.split("/").collect();
            inner = format!("{}tree {} {}\n", &inner, hash, path_split[path_split.len() - 1]);
          }

          Err(e) => {
            return Err(e);
          }
        }
      }

      for inner_blob in tree.blob.iter() {
        inner = format!("{}blob {} {}\n", inner, &inner_blob.hash, &inner_blob.name);
      }

      inner.remove(inner.len() - 1);
      inner = format!("tree {}\0{}", inner.as_bytes().len(), inner);
      let mut hasher = Sha1::new();
      hasher.input_str(&inner);
      let hash = hasher.result_str();
      match common::zlib::zlib_encoder(&inner) {
        Ok(byte) => {
          let mut tree_file = File::create(format!("./.smallgit/objects/{}", hash)).unwrap();
          tree_file.write_all(&byte).unwrap();
          return Ok(hash);
        }

        Err(_) => {
          return Err("file error".to_string());
        }
      }
    }
    if tree.hash != "" {
      return Ok(tree.hash.to_string());
    }
    return Err("hash missing".to_string());
  }

  pub fn create_commit_file(&self, hash: &str) -> Result<(), String> {
    let main_branch_paht = "./.smallgit/refs/main";
    let main_commit = fs::read_to_string(&main_branch_paht);
    match main_commit {
      Ok(main_ref) => {
        let inner = format!("tree {}\nbfore {}", hash, main_ref);
        let commit = format!("commit {}\0{}", inner.as_bytes().len(), inner);
        let mut hasher = Sha1::new();
        hasher.input_str(&commit);
        let hash = hasher.result_str();
        match common::zlib::zlib_encoder(&commit) {
          Ok(byte) => {
            let mut tree_file = File::create(format!("./.smallgit/objects/{}", hash)).unwrap();
            let mut main_file = File::create("./.smallgit/refs/main").unwrap();
            tree_file.write_all(&byte).unwrap();
            main_file.write_all(&format!("{}", hash).as_bytes()).unwrap();
            return Ok(());
          }
          Err(_) => {
            return Err("file error".to_string());
          }
        }
      }
      Err(_) => {
        return Err("main branch is abnormal".to_string());
      }
    }
  }
}
