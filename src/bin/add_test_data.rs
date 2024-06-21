use you_should_not_pass::db::Db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Db::new(&url);

    if db
        .add_new_website_account(
            "test_account1".to_string(),
            "test_password1".to_string(),
            "www.baidu.com".to_string(),
            Some("baidu".to_string()),
            Some("nothing".to_string()),
        )
        .await
        .is_err()
    {
        panic!("Failed to add new website account");
    }

    if db
        .add_new_website_account(
            "test_account2".to_string(),
            "test_password2".to_string(),
            "www.baidu.com".to_string(),
            Some("baidu".to_string()),
            Some("nothing".to_string()),
        )
        .await
        .is_err()
    {
        panic!("Failed to add new website account");
    }

    if db
        .add_new_website_account(
            "test_account3".to_string(),
            "test_password3".to_string(),
            "www.baidu.com".to_string(),
            None,
            None,
        )
        .await
        .is_err()
    {
        panic!("Failed to add new website account");
    }
}
