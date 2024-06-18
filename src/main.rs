use tokio::net::TcpListener;
use you_should_not_pass::process::process;
use you_should_not_pass::db::Db;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6123")
        .await
        .expect("Failed to bind to address");

    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Arc::new(Db::new(&url));

    loop {
        let (socket, _) = listner.accept().await.expect("Failed to accept connection");
        process(socket, db.clone()).await;
    }
}
