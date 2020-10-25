#[derive(Clone, Debug)]
pub struct IndexReaded {
  pub path: String,
  pub hex: String,
}

impl IndexReaded {
  pub fn new(path: &str, hex: &str) -> Self {
    Self {
      path: path.to_string(),
      hex: hex.to_string(),
    }
  }
}