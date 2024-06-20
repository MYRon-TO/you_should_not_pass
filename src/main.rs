use std::sync::Arc;
use tokio::net::TcpListener;
use you_should_not_pass::db::Db;
use you_should_not_pass::process::process;

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
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
