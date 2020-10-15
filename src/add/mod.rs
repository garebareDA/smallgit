pub mod add_files;

#[cfg(test)]
mod test {
  use super::super::common;
  use super::super::init;
  use super::add_files;
  use std::fs;
  #[test]
  fn add() {
    match init::init_create::create_init_file() {
      Ok(_) => {}
      Err(e) => panic!(e),
    }

    let mut paths = common::serch_dir::SerchDir::new("./");
    paths.serch_dir().unwrap();
    match add_files::write_index(paths) {
      Ok(()) => {}
      Err(s) => {
        eprintln!("{}", s);
        return;
      }
    }

    match add_files::create_objects() {
      Ok(()) => {}
      Err(s) => {
        eprintln!("{}", s);
        return;
      }
    }

    fs::remove_dir_all("./.smallgit")
      .ok()
      .expect("remove error");
  }
}
