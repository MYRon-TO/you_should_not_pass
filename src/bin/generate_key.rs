use std::{fs::File, io::Write};

use aes_gcm::{
    aead::{KeyInit, OsRng},
    Aes256Gcm, // Or `Aes128Gcm`
};
use base64::engine::general_purpose::STANDARD;
use base64::write::EncoderWriter;

fn main() {
    let key = Aes256Gcm::generate_key(OsRng);
    let key = key.as_slice();

    let mut encoder = EncoderWriter::new(Vec::new(), &STANDARD);
    encoder.write_all(key).unwrap();
    let str = encoder.finish().unwrap();
    let res = String::from_utf8(str.clone()).unwrap();

    let res = format!("KEY={}\nDATABASE_URL={}", res, "file:./you_should_not_pass/you_should_not_pass.db");

    let mut file = File::create(".env").unwrap();
    file.write_all(res.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use base64::read::DecoderReader;

    use super::*;

    #[test]
    fn test_generate_key() {
        let key = Aes256Gcm::generate_key(OsRng);
        let key = key.as_slice();

        let mut encoder = EncoderWriter::new(Vec::new(), &STANDARD);
        encoder.write_all(key).unwrap();
        let str = encoder.finish().unwrap();
        println!("{}", String::from_utf8(str.clone()).unwrap());

        let mut decoder = DecoderReader::new(str.as_slice(), &STANDARD);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();
        assert_eq!(key, decoded.as_slice());
    }
}
