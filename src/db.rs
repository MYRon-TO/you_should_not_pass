pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error;

type SqlitePool = PooledConnection<ConnectionManager<SqliteConnection>>;

struct Db {
    conn: Pool<ConnectionManager<SqliteConnection>>,
}

impl Db {
    pub fn new(url: &str) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool");
        Db { conn: pool }
    }

    fn get_conn(&self) -> Result<SqlitePool, Error> {
        if let Ok(conn) = self.conn.get() {
            Ok(conn)
        } else {
            Err(Error::NotFound)
        }
    }

    pub async fn get_user(&self, users_account: &str) -> Result<models::User, Error> {
        use schema::users::dsl::*;
        let conn = &mut self.get_conn()?;
        users.filter(account.eq(users_account)).first(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db() {
        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let db = Db::new(&url);
        let user = db.get_user("test").await.expect("Failed to get user");
        assert_eq!(user.account, "test");
    }
}
