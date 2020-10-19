use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

pub fn zlib_dencoder(decode: &str) -> String {
  let mut z = ZlibDecoder::new(decode.as_bytes());
  let bytes = z.get_mut();
  let converted: String = String::from_utf8(bytes.to_vec()).unwrap();
  return converted;
}

pub fn zlib_encoder(encode: &str) -> Result<std::vec::Vec<u8>, std::io::Error> {
  let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
  e.write_all(encode.as_bytes()).unwrap();
  let compressed_bytes = e.finish();
  return compressed_bytes;
}
