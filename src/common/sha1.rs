use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn sha1_gen(string: &str) -> String {
  let mut hasher = Sha1::new();
  hasher.input_str(string);
  let hex = hasher.result_str();
  return hex;
}