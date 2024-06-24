use std::io::{Read, Write};

use you_should_not_pass::db::Db;

use base64::engine::general_purpose::STANDARD;
use base64::read::DecoderReader;
use base64::write::EncoderWriter;

fn encode(data: String) -> String {
    let data = data.as_bytes();
    let mut encoder = EncoderWriter::new(Vec::new(), &STANDARD);
    encoder.write_all(data).unwrap();
    let str = encoder.finish().unwrap();
    String::from_utf8(str.clone()).unwrap()
}

fn _decode(data: String) -> String {
    let mut decoder = DecoderReader::new(data.as_bytes(), &STANDARD);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    String::from_utf8(decoded).unwrap()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Db::new(&url);

    if db
        .add_new_website_account(
            encode("test_account1".to_string()),
            encode("test_password1".to_string()),
            encode("www.baidu.com".to_string()),
            Some(encode("baidu".to_string())),
            Some(encode("nothing".to_string())),
        )
        .await
        .is_err()
    {
        panic!("Failed to add new website account");
    }

    if db
        .add_new_website_account(
            encode("test_account2".to_string()),
            encode("test_password2".to_string()),
            encode("https://www.baidu.com".to_string()),
            Some(encode("baidu".to_string())),
            Some(encode("nothing".to_string())),
        )
        .await
        .is_err()
    {
        panic!("Failed to add new website account");
    }

    if db
        .add_new_website_account(
            encode("test_account3".to_string()),
            encode("test_password3".to_string()),
            encode("https://www.not_exist.not_exist".to_string()),
            None,
            None,
        )
        .await
        .is_err()
    {
        panic!("Failed to add new website account");
    }
}
