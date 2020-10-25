use super::tree_git_object::CommitGet;
use super::tree_git_object::Tree;

impl CommitGet {
  pub fn object_display(&self) {
    let space = "";
    println!("{}", self.tree.name);
    self.format_object(&format!(" {}", space), &self.tree);
  }

  pub fn format_object(&self, space:&str, tree: &Tree) {
    for tree in tree.tree.iter() {
      println!("{}-{}", space, tree.name);
      for blob in tree.blob.iter() {
        println!("  {}{}", space,blob.name);
      }
      self.format_object(&format!(" {}", space), tree);
    }
  }
}