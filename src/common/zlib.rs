use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io:: {Write, Read};


pub fn zlib_dencoder(decode: &[u8]) -> String {
  println!("{:?}", decode);
  let mut z = ZlibDecoder::new(decode);
  let mut s = String::new();
  z.read_to_string(&mut s).unwrap();
  return s;
}

pub fn zlib_encoder(encode: &str) -> Result<std::vec::Vec<u8>, std::io::Error> {
  let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
  e.write_all(encode.as_bytes()).unwrap();
  let compressed_bytes = e.finish();
  return compressed_bytes;
}
