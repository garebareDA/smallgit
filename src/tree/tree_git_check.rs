use super::tree_git_object;

impl tree_git_object::CommitGet {
  pub fn check_blob(&self, path: &str, hash: &str) -> (bool, String) {
    let tree = &self.tree;
    let size = 0;
    let mut path_split: Vec<&str> = path.split("/").collect();
    path_split.remove(0);
    match self.check_blobs(tree, &path_split, size) {
      Ok(hashs) => {
        return (hashs == hash, "change".to_string());
      }

      Err(_) => {
        return (false, "create".to_string());
      }
    }
  }

  pub fn check_tree(&self, path: &str, hash: &str) -> (bool, String) {
    let tree = &self.tree;
    let size = 0;
    let mut path_split: Vec<&str> = path.split("/").collect();
    path_split.remove(0);
    match self.check_trees(tree, &path_split, size) {
      Ok(hashs) => {
        return (hashs == hash, "change".to_string());
      }

      Err(_) => {
        return (false, "create".to_string());
      }
    }
  }

  fn check_blobs(
    &self,
    tree: &tree_git_object::Tree,
    path: &Vec<&str>,
    size: usize,
  ) -> Result<String, String> {
    for inner_tree in tree.tree.iter() {
      if &inner_tree.name == path[size] {
        match self.check_blobs(inner_tree, path, size + 1) {
          Ok(hash) => {
            return Ok(hash);
          }

          Err(e) => {
            println!("{:?}", path);
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

  fn check_trees(&self, tree: &tree_git_object::Tree, path: &Vec<&str>, size: usize) -> Result<String, String>{
    let path_len = path.len() - 1;
    for inner_tree in tree.tree.iter() {
      if path_len == size && path[size] == inner_tree.name{
        return Ok(inner_tree.hash.to_string());
      }

      if path[size] == inner_tree.name {
        match self.check_trees(inner_tree, path, size + 1) {
          Ok(inner) => {
            println!("{}", inner);
            return Ok(inner);
          }

          Err(e) => {
            return Err(e);
          }
        }
      }
    }

    return Err("not found".to_string());
  }
}