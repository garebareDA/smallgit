use std::fs;

pub fn add_file(path: &str) {
  let dir_vec = read_dir(path);
}

fn read_dir(path: &str) -> Vec<String> {
  let file = fs::read_dir(path).unwrap();
  let mut path_vec: Vec<String> = Vec::new();
  for path in file {
    let path_in = path.unwrap();
    let paths = path_in.path().display().to_string();
    if path_in.file_type().unwrap().is_dir() {
      let paths_vec = read_dir(&paths);
      if !paths_vec.is_empty() {
        
      }
    }
    path_vec.push(paths);
  }
  return path_vec;
}
