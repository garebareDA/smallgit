use super::super::tree::tree_git_object::{Blob, Tree};
use super::commit_file;

use regex::Regex;

impl commit_file::CommitObject {
  pub fn extraction_dir(&mut self) {
    for index in self.index.iter() {
      let index_path = &index.path;
      let mut index_path_split: Vec<&str> = index_path.split("/").collect();
      index_path_split.remove(index_path_split.len() - 1);
      let connect = index_path_split.join("/");
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
    let mut tree_root = Tree::new("/", "");
    let tree = self.tree_dir(1, "/");
    tree_root.blob = self.insert_blob("/");
    tree_root.tree = tree;
    return tree_root;
  }

  fn tree_dir(&self, size: usize, pearent: &str) -> Vec<Tree> {
    let mut tree_vec: Vec<Tree> = Vec::new();
    for index in 0..self.tree_dir.len() {
      let path_split: Vec<&str> = self.tree_dir[index].split("/").collect();
      if path_split.len() == size {
        let name_dir = &self.tree_dir[index];
        let name_split: Vec<&str> = name_dir.split("/").collect();
        let name = name_split[name_split.len() - 1];
        let mut tree = Tree::new(name, "");
        tree.blob = self.insert_blob(&name_dir);
        tree.tree = self.tree_dir(size + 1, name_dir);
        let re = Regex::new(&format!(r"^{}", pearent)).unwrap();
        match re.captures(name_dir) {
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

  fn insert_blob(&self, tree: &str) -> Vec<Blob> {
    let mut blob_vec: Vec<Blob> = Vec::new();
    for index in self.index.iter() {
      let dir = tree;
      let file = &index.path;
      let hex = &index.hex;
      let status = &index.status;
      let file_split: Vec<&str> = file.split("/").collect();
      let file_name = file_split[file_split.len() - 1];

      if dir == "/" && file_split.len() == 1 {
        let blob = Blob::new(file_name, hex, status);
        blob_vec.push(blob);
        continue;
      }

      let re = Regex::new(&format!(r"^{}/{}", dir, file_name)).unwrap();
      match re.captures(file) {
        Some(_) => {
          let blob = Blob::new(file_name, hex, status);
          blob_vec.push(blob);
        }
        None => {}
      }
    }
    return blob_vec;
  }

  pub fn comparsion_tree(&self, root_tree: &mut Tree, main_tree: &mut Tree) {
    for blob in root_tree.blob.iter() {
      if main_tree.blob.is_empty() {
        main_tree.blob.push(blob.clone());
        main_tree.is_edit = true;
        continue;
      }

      for index in 0..main_tree.blob.len() {
        if main_tree.blob[index].name == blob.name {
          let status = &blob.status;
          if status == "remove" {
            main_tree.blob.remove(index);
            break;
          }
          main_tree.blob[index] = blob.clone();
          main_tree.is_edit = true;
          break;
        }

        if index == main_tree.blob.len() - 1 {
          main_tree.blob.push(blob.clone());
          main_tree.is_edit = true;
          break;
        }
      }
    }

    for tree in root_tree.tree.iter() {
      if main_tree.tree.is_empty() {
        let mut push_tree = tree.clone();
        self.change_edit(&mut push_tree);
        main_tree.is_edit = true;
        main_tree.tree.push(push_tree);
      }

      for index in 0..main_tree.tree.len() {
        if main_tree.tree[index].name == tree.name {
          self.comparsion_tree(&mut tree.clone(), &mut main_tree.tree[index]);
          let tree = &main_tree.tree[index];
          if tree.tree.is_empty() && tree.blob.is_empty(){
            main_tree.tree.remove(index);
          }
          main_tree.is_edit = true;
          break;
        }

        if index == main_tree.tree.len() - 1 {
          let mut push_tree = tree.clone();
          self.change_edit(&mut push_tree);
          main_tree.is_edit = true;
          main_tree.tree.push(push_tree);
          self.comparsion_tree(&mut tree.clone(), &mut main_tree.tree[index]);
          break;
        }
      }
    }
  }

  fn change_edit(&self, tree: &mut Tree) {
    tree.is_edit = true;
    for index in 0..tree.tree.len() {
      tree.tree[index].is_edit = true;
      self.change_edit(&mut tree.tree[index]);
    }
  }
}
