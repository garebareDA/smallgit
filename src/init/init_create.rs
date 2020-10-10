use std::fs;
use std::fs::File;

pub fn create_init_file() -> Result<(), String> {
  match fs::create_dir("./.smallgit") {
      Ok(_) => {}
      Err(_) => {
          return Err("failed to create dir".to_string());
      }
  }

  match File::create("./.smallgit/index") {
      Ok(_) => {}
      Err(_) => {
          return Err("failed to create  file".to_string());
      }
  }

  match fs::create_dir("./.smallgit/refs") {
      Ok(_) => {}
      Err(_) => {
          return Err("failed to create dir".to_string());
      }
  }

  match File::create("./.smallgit/refs/main") {
    Ok(_) => {}
    Err(_) => {
        return Err("failed to create  file".to_string());
    }
}

  match fs::create_dir("./.smallgit/objects") {
      Ok(_) => {}
      Err(_) => {
          return Err("failed to create dir".to_string());
      }
  }

  return Ok(());
}
