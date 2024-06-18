use std::error::Error;
use tokio::net::TcpStream;

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
/// "ACTION_ID OTHER_MESSAGE"
///
/// for example:
/// "0 my_password"
///
/// > The ACTION_ID is the index of the Action enum in task.rs
/// > here is the index:
/// > - 0: CheckIdentity
/// > - 1: Login
/// > - 2: AddWebsiteAccount
/// > - 3: ChangeWebsiteAccount
/// > - 4: DeleteWebsiteAccount
/// > - 5: CheckDeadLink
///
pub async fn read_request(stream: &TcpStream) -> Result<Action, Box<dyn Error>> {
    stream.readable().await?;
    let mut buffer = [0; 4096];
    match stream.try_read(&mut buffer) {
        Ok(0) => Err("Failed to read from socket".into()),
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            let parts: Vec<&str> = request.split_whitespace().collect();
            Ok(pack_action(parts)?)
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Err("Blocked".into()),
        Err(e) => Err(e.into()),
    }
}

fn pack_action(parts: Vec<&str>) -> Result<Action, Box<dyn Error>> {
    let action_id = parts[0].parse::<i32>()?;
    match action_id {
        0 => {
            let password = parts.get(1).ok_or("Password is missing")?.to_string();
            Ok(Action::CheckIdentity { password })
        }
        1 => Ok(Action::GetInfo),
        // 2 => {
        //     let website_id = parts
        //         .get(1)
        //         .ok_or("Website id is missing")?
        //         .parse::<i32>()?;
        //     Ok(Action::GetWebsiteAccountPassword { website_id })
        // }
        2 => {
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
        3 => {
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
        4 => {
            let website_id = parts
                .get(1)
                .ok_or("Website id is missing")?
                .parse::<i32>()?;
            Ok(Action::DeleteWebsiteAccount { website_id })
        }
        5 => Ok(Action::CheckDeadLink),
        _ => Err("Invalid action id".into()),
    }
}
