use std::fs::File;
use std::io::Read;

use super::super::common::zlib;
use super::tree_git_object;

impl tree_git_object::Commit {
  pub fn tree_go_back(&mut self) -> Result<(), String> {
    let hash = &self.tree.hash;
    match self.tree_file_judge(hash) {
      Ok((blob, trees)) => {
        self.tree.blob = blob;
        self.tree.tree = trees;
        return Ok(());
      }
      Err(s) => {
        return Err(s);
      }
      
    }
  }

  pub fn tree_file_judge(
    &self,
    hash: &str,
  ) -> Result<(Vec<tree_git_object::Blob>, Vec<tree_git_object::Tree>), String> {
    let mut blob_vec: Vec<tree_git_object::Blob> = Vec::new();
    let mut tree_vec: Vec<tree_git_object::Tree> = Vec::new();
    let path = &format!("./.smallgit/objects/{}", hash);
    let file = File::open(path);
    let mut buffer = Vec::new();
    match file {
      Ok(mut file) => {
        let _ = file.read_to_end(&mut buffer).unwrap();
        let decoded = zlib::zlib_dencoder(&buffer);
        let decoded_split: Vec<&str> = decoded.split("\0").collect();
        for line in decoded_split[1].lines() {
          let line_split: Vec<&str> = line.split(" ").collect();
          match line_split[0] {
            "blob" => {
              let blob = tree_git_object::Blob::new(line_split[1],line_split[2]);
              blob_vec.push(blob);
            }
            "tree" => {
              let mut tree = tree_git_object::Tree::new(line_split[1], line_split[2]);
              match self.tree_file_judge(line_split[2]) {
                Ok((blob, trees)) => {
                  tree.blob = blob;
                  tree.tree = trees;
                }
                Err(s) => {
                  return Err(s);
                }
              }
              tree_vec.push(tree);
            }
            _ => {
              return Err("git object error".to_string());
            }
          }
        }
      }
      Err(_) => {
        return Err("git object not found".to_string());
      }
    }

    return Ok((blob_vec, tree_vec));
  }
}
