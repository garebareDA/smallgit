pub mod add;
pub mod commit;
pub mod common;
pub mod init;
pub mod tree;

#[cfg(test)]
mod test {
  use super::add;
  use super::commit;
  use super::common;
  use super::init;
  use super::tree;
  use std::fs;
  #[test]
  fn git_init() {
    match init::init_create::create_init_file() {
      Ok(_) => {}
      Err(e) => panic!(e),
    }

    fs::remove_dir_all("./.smallgit")
      .ok()
      .expect("remove error");
  }

  #[test]
  fn serch() {
    match init::init_create::create_init_file() {
      Ok(_) => {}
      Err(_) => {}
    }

    let mut paths = common::serch_dir::SerchDir::new("./");
    paths.serch_dir().unwrap();
    match add::add_files::write_index(paths) {
      Ok(()) => {}
      Err(s) => {
        panic!(s);
      }
    }

    match add::add_files::create_objects() {
      Ok(()) => {}
      Err(s) => {
        panic!(s);
      }
    }

    fs::remove_dir_all("./.smallgit")
      .ok()
      .expect("remove error");
  }

  #[test]
  fn git_add() {
    match init::init_create::create_init_file() {
      Ok(_) => {}
      Err(_) => {}
    }

    let mut paths = common::serch_dir::SerchDir::new("./");
    paths.serch_dir().unwrap();
    match add::add_files::write_index(paths) {
      Ok(()) => {}
      Err(s) => {
        eprintln!("{}", s);
        return;
      }
    }

    match add::add_files::create_objects() {
      Ok(()) => {}
      Err(s) => {
        panic!(s);
      }
    }

    fs::remove_dir_all("./.smallgit")
      .ok()
      .expect("remove error");
  }

  #[test]
  fn git_commit() {
    match init::init_create::create_init_file() {
      Ok(_) => {}
      Err(e) => panic!(e),
    }

    let mut paths = common::serch_dir::SerchDir::new("./");
    paths.serch_dir().unwrap();
    match add::add_files::write_index(paths) {
      Ok(()) => {}
      Err(s) => {
        eprintln!("{}", s);
        return;
      }
    }

    match add::add_files::create_objects() {
      Ok(()) => {}
      Err(s) => {
        panic!(s);
      }
    }

    let mut commit = commit::commit_file::CommitObject::new();
    let mut paths = common::serch_dir::SerchDir::new("./");
    paths.serch_dir().unwrap();
    match commit.commit_file(&paths.get_paths_dir()) {
      Ok(_) => {}
      Err(s) => {
        panic!(s);
      }
    }

    fs::remove_dir_all("./.smallgit")
      .ok()
      .expect("remove error");
  }

  #[test]
  fn git_tree() {
    match init::init_create::create_init_file() {
      Ok(_) => {}
      Err(_) => {}
    }

    let mut paths = common::serch_dir::SerchDir::new("./");
    paths.serch_dir().unwrap();
    match add::add_files::write_index(paths) {
      Ok(()) => {}
      Err(s) => {
        eprintln!("{}", s);
        return;
      }
    }

    match add::add_files::create_objects() {
      Ok(()) => {}
      Err(_) => {}
    }

    let mut commit = commit::commit_file::CommitObject::new();
    let mut paths = common::serch_dir::SerchDir::new("./");
    paths.serch_dir().unwrap();
    match commit.commit_file(&paths.get_paths_dir()) {
      Ok(_) => {}
      Err(_) => {}
    }

    let mut tree = tree::tree_git_object::Commit::new();
    match tree.tree_main() {
      Ok(_) => {}
      Err(e) => {
        panic!(e);
      }
    }

    fs::remove_dir_all("./.smallgit")
      .ok()
      .expect("remove error");
  }
}
