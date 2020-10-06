pub mod init_create;

#[cfg(test)]
mod test {
  use std::fs;
  use super::init_create;

  #[test]
  fn init() {
    match init_create::create_init_file() {
      Ok(_) => {}
      Err(e) => panic!(e),
    }
    fs::remove_dir_all("./.smallgit")
      .ok()
      .expect("remove error");
  }
}
