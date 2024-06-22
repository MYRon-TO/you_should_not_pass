use crate::db::models::WebsiteAccountWithDeadLink;

pub enum ProError {
    DbError(diesel::result::Error),
    IdentityError(pam::PamError),
}

pub enum ProOk {
    Ack,
    Info(Vec<WebsiteAccountWithDeadLink>),
    DeadLink(Vec<(i32, bool)>),
}
