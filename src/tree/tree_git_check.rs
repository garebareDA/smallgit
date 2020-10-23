use super::tree_git_object;

impl tree_git_object::Commit {
  pub fn check(&self, path: &str, hash: &str) -> bool {
    let tree = &self.tree;
    let size = 0;
    let mut path_split:Vec<&str> = path.split("/").collect();
    path_split.remove(0);
    match self.check_tree(tree, &path_split, size) {
      Ok(hashs) => {
        return hashs == hash;
      }

      Err(_) => {
        return false;
      }
    }
  }

  fn check_tree(&self, tree:&tree_git_object::Tree, path:&Vec<&str>, size:usize) -> Result<String, String>{
    for inner_tree in tree.tree.iter() {
      if &inner_tree.name == path[size] {
        match self.check_tree(inner_tree, path, size + 1) {
          Ok(hash) => {
            return Ok(hash);
          }

          Err(e) => {
            return Err(e);
          }
        }
      }
    }

    for blob in tree.blob.iter() {
      if &blob.name == path[size] {
        return Ok(blob.hash.to_string());
      }
    }

    return Err("not found".to_string());
  }
}