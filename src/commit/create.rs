use super::super::common;
use super::super::tree::tree_git_object::{Blob, Tree};
use super::commit_file;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::File;
use std::io::Write;

impl commit_file::CommitObject {
  pub fn create_tree_file(&mut self) -> Result<String, String>{
    let mut tree = self.tree.clone();
    match self.tree_write(&mut tree) {
      Ok(hash) => {
        self.tree = tree;
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
            inner = format!("{}tree {} {}\n", &inner, hash, tree.tree[index].name);
          }

          Err(e) => {
            return Err(e);
          }
        }
      }

      for inner_blob in tree.blob.iter() {
        inner = format!("{}blob {} {}\n", inner, inner_blob.hash, inner_blob.name);
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
}
