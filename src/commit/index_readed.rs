#[derive(Clone, Debug)]
pub struct IndexReaded {
  pub path: String,
  pub hex: String,
  pub status: String,
}

impl IndexReaded {
  pub fn new(path: &str, hex: &str, status:&str) -> Self {
    Self {
      path: path.to_string(),
      hex: hex.to_string(),
      status:status.to_string(),
    }
  }
}