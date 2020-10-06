pub mod init_create;

use std::fs;
#[test]
fn init() {
  match init_create::create_init_file() {
    Ok(_) => {}
    Err(e) => panic!(e),
  }
  fs::remove_dir_all("./.smallgit");
}
