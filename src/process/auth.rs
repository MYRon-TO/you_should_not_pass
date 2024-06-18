use pam::Authenticator;
use pam::PamResult;

pub async fn authourize(password: String) -> PamResult<()>{
    let mut auth = Authenticator::with_password("login")?;
    let username = whoami::username();
    auth.get_handler().set_credentials(username, password);
    auth.authenticate()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_authourize() {
        dotenv::dotenv().ok();
        let password = std::env::var("PAM_PASSWORD").unwrap();

        let result = authourize(password).await;
        println!("{:?}", result);

        assert!(result.is_ok());
    }
}
