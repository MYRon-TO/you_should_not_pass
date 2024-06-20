use std::error::Error;
use tokio::net::TcpStream;

#[derive(Debug, PartialEq)]
pub enum Action {
    CheckIdentity {
        password: String,
    },
    // user_account
    GetInfo,
    // website_account
    // GetWebsiteAccountPassword {
    //     website_id: i32,
    // },
    AddWebsiteAccount {
        account: String,
        password: String,
        site_url: String,
        site_name: Option<String>,
        note: Option<String>,
    },
    ChangeWebsiteAccount {
        id: i32,
        new_account: String,
        new_password: String,
        new_site_name: Option<String>,
        new_site_url: String,
        new_note: Option<String>,
    },
    DeleteWebsiteAccount {
        website_id: i32,
    },
    // check_dead_link
    CheckDeadLink,
}

/// read the request from the socket and return a task
///
/// Here is the TCP format:
/// "ACTION\tOTHER_MESSAGE"
///
/// for example:
/// - `"CheckIdentity\tmy_password"`
/// - `"AddWebsiteAccount\tmy_account\tmy_password\tmy_site_url\tmy_site_name\tmy_note"`
///
/// ## Here is the list of action:
/// > - CheckIdentity
/// > - Login
/// > - AddWebsiteAccount
/// > - ChangeWebsiteAccount
/// > - DeleteWebsiteAccount
/// > - CheckDeadLink
///
pub async fn read_request(stream: &TcpStream) -> Result<Action, Box<dyn Error>> {
    stream.readable().await?;
    let mut buffer = [0; 4096];
    match stream.try_read(&mut buffer) {
        Ok(0) => Err("Failed to read from socket".into()),
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            let parts: Vec<&str> = request.split('\t').collect();
            Ok(pack_action(parts)?)
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Err("Blocked".into()),
        Err(e) => Err(e.into()),
    }
}

fn pack_action(parts: Vec<&str>) -> Result<Action, Box<dyn Error>> {
    let action = parts[0];
    match action {
        "CheckIdentity" => {
            let password = parts.get(1).ok_or("Password is missing")?.to_string();
            Ok(Action::CheckIdentity { password })
        }
        "Login" => Ok(Action::GetInfo),
        "AddWebsiteAccount" => {
            let account = parts.get(1).ok_or("Account is missing")?.to_string();
            let password = parts.get(2).ok_or("Password is missing")?.to_string();
            let site_url = parts.get(3).ok_or("Site URL is missing")?.to_string();
            let site_name = parts.get(4).map(|s| s.to_string());
            let note = parts.get(5).map(|s| s.to_string());
            Ok(Action::AddWebsiteAccount {
                account,
                password,
                site_url,
                site_name,
                note,
            })
        }
        "ChangeWebsiteAccount" => {
            let id = parts
                .get(1)
                .ok_or("Website id is missing")?
                .parse::<i32>()?;
            let new_account = parts.get(2).ok_or("Account is missing")?.to_string();
            let new_password = parts.get(3).ok_or("Password is missing")?.to_string();
            let new_site_name = parts.get(4).map(|s| s.to_string());
            let new_site_url = parts.get(5).ok_or("Site URL is missing")?.to_string();
            let new_note = parts.get(6).map(|s| s.to_string());
            Ok(Action::ChangeWebsiteAccount {
                id,
                new_account,
                new_password,
                new_site_name,
                new_site_url,
                new_note,
            })
        }
        "DeleteWebsiteAccount" => {
            let website_id = parts
                .get(1)
                .ok_or("Website id is missing")?
                .parse::<i32>()?;
            Ok(Action::DeleteWebsiteAccount { website_id })
        }
        "CheckDeadLink" => Ok(Action::CheckDeadLink),
        _ => Err("Invalid Action".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_action() {
        let parts = vec!["CheckIdentity", "my_password"];
        let action = pack_action(parts).unwrap();
        assert_eq!(
            action,
            Action::CheckIdentity {
                password: "my_password".to_string()
            }
        );

        let parts = vec!["Login"];
        let action = pack_action(parts).unwrap();
        assert_eq!(action, Action::GetInfo);

        let parts = vec![
            "AddWebsiteAccount",
            "my_account",
            "my_password",
            "my_site_url",
            "my_site_name",
            "my_note",
        ];
        let action = pack_action(parts).unwrap();
        assert_eq!(
            action,
            Action::AddWebsiteAccount {
                account: "my_account".to_string(),
                password: "my_password".to_string(),
                site_url: "my_site_url".to_string(),
                site_name: Some("my_site_name".to_string()),
                note: Some("my_note".to_string()),
            }
        );

        let parts = vec![
            "ChangeWebsiteAccount",
            "1",
            "my_account",
            "my_password",
            "my_site_name",
            "my_site_url",
            "my_note",
        ];
        let action = pack_action(parts).unwrap();
        assert_eq!(
            action,
            Action::ChangeWebsiteAccount {
                id: 1,
                new_account: "my_account".to_string(),
                new_password: "my_password".to_string(),
                new_site_name: Some("my_site_name".to_string()),
                new_site_url: "my_site_url".to_string(),
                new_note: Some("my_note".to_string()),
            }
        );

        let parts = vec!["DeleteWebsiteAccount", "1"];
        let action = pack_action(parts).unwrap();
        assert_eq!(action, Action::DeleteWebsiteAccount { website_id: 1 });

        let parts = vec!["CheckDeadLink"];
        let action = pack_action(parts).unwrap();
        assert_eq!(action, Action::CheckDeadLink);
    }
}
