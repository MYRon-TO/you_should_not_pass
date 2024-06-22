use crate::db::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::website_account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WebsiteAccount {
    pub id: Option<i32>,
    pub account: String,
    pub password: String,
    pub site_url: String,
    pub site_name: Option<String>,
    pub note: Option<String>,
}

pub struct WebsiteAccountWithDeadLink {
    pub id: Option<i32>,
    pub account: String,
    pub password: String,
    pub site_url: String,
    pub site_name: Option<String>,
    pub note: Option<String>,
    pub dead_link: bool,
}
