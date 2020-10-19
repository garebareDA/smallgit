use std::fs;
use std::fs::File;
use std::io:: Read;

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
  match get_refs_main() {
    Ok(main_ref) => match get_commit_object(&main_ref) {
      Ok(hex) => {
        println!("{}", hex)
      }
      Err(e) => {
        return Err(e);
      }
    },
    Err(_) => {
      return Err("main branch is abnormal".to_string());
    }
  }

  return Ok(());
}

fn get_refs_main() -> Result<String, String> {
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
fn get_commit_object(hash: &str) -> Result<String, String> {
  let commit_tree_path = &format!("./.smallgit/objects/{}", hash);
  let file = File::open(commit_tree_path);
  let mut buffer = Vec::new();
  match file {
    Ok(mut file) => {
      let _ = file.read_to_end(&mut buffer).unwrap();
      let decoded = zlib::zlib_dencoder(&buffer);
      let decoded_split:Vec<&str> = decoded.split("\0").collect();
      let tree_split:Vec<&str> = decoded_split[1].split(" ").collect();
      return Ok(tree_split[1].to_string());
    }
    Err(_) => {
      return Err("commit objects not found".to_string());
    }
  }
}
