use super::super::tree::tree_git_object::{Blob, Tree};
use super::commit_file;

use regex::Regex;

impl commit_file::CommitObject {
  pub fn extraction_dir(&mut self) {
    for index in self.index.iter() {
      let index_path = &index.path;
      let mut index_path_split: Vec<&str> = index_path.split("/").collect();
      index_path_split.remove(index_path_split.len() - 1);
      let connect = index_path_split.connect("/");
      if connect != "" {
        self.tree_dir.push(connect.clone());
      } else if !index_path_split.is_empty() {
        self.tree_dir.push(index_path_split[0].to_string());
      }

      let mut paths = "".to_string();
      for (index, path) in index_path_split.iter().enumerate() {
        if index == 0 {
          self.tree_dir.push(path.to_string());
          paths = format!("{}", &path);
          continue;
        }
        paths = format!("{}/{}", &paths, &path);
        self.tree_dir.push(paths.to_string());
      }
      self.tree_dir.sort();
      self.tree_dir.dedup();
    }
  }

  pub fn generate_tree(&mut self) -> Tree {
    if self.tree.name == "" {
      self.tree.name = "/".to_string();
    }
    let tree = self.tree_dir(1, "/");
    let mut tree_root = Tree::new("/", "");
    self.insert_blob(&mut tree_root);
    tree_root.tree = tree;
    return tree_root;
  }

  fn tree_dir(&self, size: usize, pearent: &str) -> Vec<Tree> {
    let mut tree_vec: Vec<Tree> = Vec::new();
    for index in 0..self.tree_dir.len() - 1 {
      let path_split: Vec<&str> = self.tree_dir[index].split("/").collect();
      if path_split.len() == size {
        let name = &self.tree_dir[index];
        let mut tree = Tree::new(name, "");
        self.insert_blob(&mut tree);
        tree.tree = self.tree_dir(size + 1, name);
        let re = Regex::new(&format!(r"^{}", pearent)).unwrap();
        match re.captures(name) {
          Some(_) => {
            tree_vec.push(tree);
          }

          None => {
            if size == 1 {
              tree_vec.push(tree)
            }
          }
        }
      }
    }
    return tree_vec;
  }

  fn insert_blob(&self, tree: &mut Tree) {
    let mut blob_vec: Vec<Blob> = Vec::new();
    for index in self.index.iter() {
      let dir = &tree.name;
      let file = &index.path;
      let hex = &index.hex;
      let file_split: Vec<&str> = file.split("/").collect();
      let file_name = file_split[file_split.len() - 1];

      if dir == "/" && file_split.len() == 1 {
        let blob = Blob::new(file_name, hex);
        blob_vec.push(blob);
        continue;
      }

      let re = Regex::new(&format!(r"^{}/{}", dir, file_name)).unwrap();
      match re.captures(file) {
        Some(_) => {
          let blob = Blob::new(file_name, hex);
          blob_vec.push(blob);
        }
        None => {}
      }
    }
    tree.blob = blob_vec;
  }

  pub fn comparsion_tree(&self, root_tree: &mut Tree, main_tree:&mut Tree) {
    for index in 0..root_tree.tree.len() - 1{
      self.comparsion_tree(&mut root_tree.tree[index], &mut main_tree.tree[index]);
    }
  }
}