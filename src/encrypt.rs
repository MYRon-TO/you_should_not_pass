use std::io::{Read, Write};

use aes_gcm::{
    aead::{self, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};

use base64::engine::general_purpose::STANDARD;
use base64::read::DecoderReader;
use base64::write::EncoderWriter;

pub async fn encrypt(password: String) -> Result<String, aead::Error> {
    let key = get_key();
    let key = Key::<Aes256Gcm>::from_slice(&key);

    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(&nonce, password.as_bytes().as_ref())?;

    let result = format!(
        "{}:{}",
        encode(ciphertext),
        encode(nonce.as_slice().to_vec())
    );

    Ok(result)
}

pub async fn decrypt(data: String) -> Result<String, aead::Error> {
    let key = get_key();
    let key = Key::<Aes256Gcm>::from_slice(&key);

    let cipher = Aes256Gcm::new(key);
    let mut data = data.split(':');

    let chiphertext = data.next().unwrap();
    let nonce = data.next().unwrap();

    let chiphertext = decode(chiphertext.to_string());
    let nonce = decode(nonce.to_string());

    let nonce = aes_gcm::Nonce::from_slice(&nonce);

    let plaintext = cipher.decrypt(nonce, chiphertext.as_ref())?;

    Ok(String::from_utf8(plaintext).unwrap())
}

fn get_key() -> [u8; 32] {
    dotenv::dotenv().ok();
    let key = std::env::var("KEY").expect("Here is no key");

    let key = decode(key);

    key.as_slice().try_into().unwrap()
}

fn encode(data: Vec<u8>) -> String {
    let mut encoder = EncoderWriter::new(Vec::new(), &STANDARD);
    encoder.write_all(&data).unwrap();
    let str = encoder.finish().unwrap();
    String::from_utf8(str.clone()).unwrap()
}

fn decode(data: String) -> Vec<u8> {
    let mut decoder = DecoderReader::new(data.as_bytes(), &STANDARD);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encrypt_and_decrypt() {
        let password = "password".to_string();
        let encrypted = encrypt(password.clone()).await.unwrap();
        let decrypted = decrypt(encrypted).await.unwrap();

        assert_eq!(password, decrypted);
    }
}
