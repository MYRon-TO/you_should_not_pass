use diesel::prelude::*;
use crate::db::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub account: String,
    pub password: String,
    pub identity: bool
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::website_account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WebsiteAccount {
    pub id: Option<i32>,
    pub account: String,
    pub password: String,
    pub site_name: String,
    pub site_url: String,
    pub note: Option<String>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users_account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserAccount {
    pub account_id: i32,
    pub user_id: i32
}
