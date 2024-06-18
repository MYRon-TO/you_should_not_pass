pub enum Error {
    DbError(diesel::result::Error),
    IdentityError(pam::PamError),
}
