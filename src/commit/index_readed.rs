#[derive(Clone, Debug)]
pub struct IndexReaded {
  path: String,
  hex: String,
}

impl IndexReaded {
  pub fn new(path: &str, hex: &str) -> Self {
    Self {
      path: path.to_string(),
      hex: hex.to_string(),
    }
  }

  pub fn get_path(self) -> String {
    self.path
  }

  pub fn get_hex(self) -> String {
    self.hex
  }
}