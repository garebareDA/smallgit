use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::common::zlib;

struct Tree {
  name: String,
  hash: String,
  blob: Vec<Blob>,
  tree: Vec<Tree>,
}

struct Blob {
  name: String,
  hash: String,
}

pub fn commit_tree_object() -> Result<(), String> {
  match refs_get_main() {
    Ok(main_ref) => {
      commit_get_tree(&main_ref);
    }
    Err(_) => {
      return Err("main branch is abnormal".to_string());
    }
  }

  return Ok(());
}

fn refs_get_main() -> Result<String, String> {
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

fn commit_get_tree(hash: &str) {
  let commit_tree_path = format!("./.smallgit/objects/{}", hash);
  let main_commit = fs::read_to_string(&commit_tree_path);
  match main_commit {
    Ok(commit_zlib) => {
      let decode = zlib::zlib_dencoder(&commit_zlib);
    }

    Err(_) => {

    }
  }
}

