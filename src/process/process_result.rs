use crate::db::models::WebsiteAccount;

pub enum ProError {
    DbError(diesel::result::Error),
    IdentityError(pam::PamError),
}

pub enum ProOk{
    Ack,
    Info(Vec<WebsiteAccount>),
    DeadLink(Vec<(i32, bool)>),
}
