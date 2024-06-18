pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::query_dsl::methods::SelectNullableDsl;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error;

type SqlitePool = PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct Db {
    conn: Pool<ConnectionManager<SqliteConnection>>,
}

// Connection
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
}

// SQL
impl Db {
    pub async fn add_new_website_account(
        &self,
        new_account: String,
        new_password: String,
        new_site_url: String,
        new_site_name: Option<String>,
        new_note: Option<String>,
    ) -> Result<(), diesel::result::Error> {
        use schema::website_account::dsl::*;

        let mut conn = self.get_conn()?;
        let new_website_account = models::WebsiteAccount {
            id: None,
            account: new_account,
            password: new_password,
            site_url: new_site_url,
            site_name: new_site_name,
            note: new_note,
        };

        diesel::insert_into(website_account)
            .values(&new_website_account)
            .execute(&mut conn)?;
        Ok(())
    }

    pub async fn update_website_account(
        &self,
        website_id: i32,
        new_account: String,
        new_password: String,
        new_site_name: Option<String>,
        new_site_url: String,
        new_note: Option<String>,
    ) -> Result<(), diesel::result::Error> {
        use schema::website_account::dsl::*;

        let mut conn = self.get_conn()?;
        diesel::update(website_account.filter(id.eq(website_id)))
            .set((
                account.eq(new_account),
                password.eq(new_password),
                site_name.eq(new_site_name),
                site_url.eq(new_site_url),
                note.eq(new_note),
            ))
            .execute(&mut conn)?;
        Ok(())
    }

    pub async fn delete_website_account(
        &self,
        website_id: i32,
    ) -> Result<(), diesel::result::Error> {
        use schema::website_account::dsl::*;

        let mut conn = self.get_conn()?;
        diesel::delete(website_account.filter(id.eq(website_id))).execute(&mut conn)?;
        Ok(())
    }

    pub async fn get_website_account_password(
        &self,
        website_id: i32,
    ) -> Result<Option<String>, diesel::result::Error> {
        use schema::website_account::dsl::*;

        let mut conn = self.get_conn()?;
        let result = website_account
            .filter(id.eq(website_id))
            .select(password)
            .first::<String>(&mut conn)
            .optional()?;

        Ok(result)
    }

    pub async fn get_all_website_account(
        &self,
    ) -> Result<Vec<models::WebsiteAccount>, diesel::result::Error> {
        use schema::website_account::dsl::*;

        let mut conn = self.get_conn()?;
        let result = website_account.load::<models::WebsiteAccount>(&mut conn)?;

        Ok(result)
    }

    pub async fn get_all_id_and_url(
        &self,
    ) -> Result<Vec<(String, i32)>, diesel::result::Error> {
        use schema::website_account::dsl::*;

        let mut conn = self.get_conn()?;
        let result = website_account
            .select((site_url, id.nullable()))
            .load::<(String, Option<i32>)>(&mut conn)?;

        let mut result_vec = vec![];
        for (url, some_id) in result {
            if let Some(t) = some_id {
                result_vec.push((url.clone(), t));
            }
        }
        Ok(result_vec)
    }

    pub async fn get_website_id_by_account(
        &self,
        account_to_search: &str,
    ) -> Result<Option<i32>, diesel::result::Error> {
        use schema::website_account::dsl::*;

        let mut conn = self.get_conn()?;
        let result = website_account
            .filter(account.eq(account_to_search))
            .select(id)
            .first::<Option<i32>>(&mut conn)
            .optional()?;

        // to avoid nested Option<>
        Ok(result.flatten())
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

        if db
            .add_new_website_account(
                "test_account".to_string(),
                "test_password".to_string(),
                "www.baidu.com".to_string(),
                Some("baidu".to_string()),
                Some("nothing".to_string()),
            )
            .await
            .is_err()
        {
            panic!("Failed to add new website account");
        }

        match db.get_website_id_by_account("test_account").await {
            Ok(Some(id)) => {
                if (db.get_website_account_password(id).await).is_err() {
                    panic!("Failed to get website account password");
                }
            }
            _ => panic!("Failed to get website id by account"),
        }
    }
}
