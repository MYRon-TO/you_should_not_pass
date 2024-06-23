mod action;
mod auth;
mod check_dead_link;
mod process_result;

use crate::db::Db;
use action::*;
use check_dead_link::{check_dead_link, check_dead_link_info};
use process_result::{ProError, ProOk};
use std::sync::Arc;
use tokio::net::TcpStream;

/// Process the socket
pub async fn process(socket: TcpStream, db: Arc<Db>) {
    // Process the socket
    let action = match read_request(&socket).await {
        Ok(action) => action,
        Err(e) => {
            eprintln!("Failed to read request: {}", e);
            return;
        }
    };

    let result = handle_action(action, db).await;
    if (answer_request(&socket, result).await).is_err() {
        eprintln!("Failed to answer request");
    }
}

async fn handle_action(action: Action, db: Arc<Db>) -> Result<ProOk, ProError> {
    match action {
        Action::CheckIdentity { password } => {
            // Check the password
            if let Err(e) = auth::authourize(password).await {
                return Err(ProError::IdentityError(e));
            }
            Ok(ProOk::Ack)
        }

        Action::GetInfo => {
            // GetInfo
            let list = match db.get_all_website_account().await {
                Ok(list) => list,
                Err(e) => return Err(ProError::DbError(e)),
            };

            let result = check_dead_link_info(list).await;

            Ok(ProOk::Info(result))
        }

        Action::AddWebsiteAccount {
            account,
            password,
            site_url,
            site_name,
            note,
        } => {
            // Add the website account
            if let Err(e) = db
                .add_new_website_account(account, password, site_url, site_name, note)
                .await
            {
                return Err(ProError::DbError(e));
            }
            Ok(ProOk::Ack)
        }
        Action::ChangeWebsiteAccount {
            id,
            new_account,
            new_password,
            new_site_name,
            new_site_url,
            new_note,
        } => {
            // Change the website account
            if let Err(e) = db
                .update_website_account(
                    id,
                    new_account,
                    new_password,
                    new_site_name,
                    new_site_url,
                    new_note,
                )
                .await
            {
                return Err(ProError::DbError(e));
            }
            Ok(ProOk::Ack)
        }
        Action::DeleteWebsiteAccount { website_id } => {
            // Delete the website account
            if let Err(e) = db.delete_website_account(website_id).await {
                return Err(ProError::DbError(e));
            }
            Ok(ProOk::Ack)
        }
        Action::CheckDeadLink => {
            // Check the dead link
            match db.get_all_id_and_url().await {
                Ok(id_and_url) => {
                    // todo
                    let list = check_dead_link(id_and_url).await;
                    Ok(ProOk::DeadLink(list))
                }
                Err(e) => Err(ProError::DbError(e)),
            }
        }
    }
}

/// Ack: 0
/// Info: 1
/// DeadLink: 2
/// IdentityError: 3
/// DbError: 4
async fn answer_request(
    socket: &TcpStream,
    result: Result<ProOk, ProError>,
) -> Result<(), std::io::Error> {
    let response = match result {
        Ok(ProOk::Ack) => "0".to_string(),
        Ok(ProOk::Info(list)) => {
            let mut response: String = "1".to_string();
            for item in list {
                let site_name: &str = if let Some(x) = &item.site_name {
                    x.as_str()
                } else {
                    ""
                };

                let note: &str = if let Some(x) = &item.note {
                    x.as_str()
                } else {
                    ""
                };

                let id = if let Some(x) = item.id { x } else { -1 };
                let is_dead = if item.dead_link { "0" } else { "1" };

                let res = format!(
                    "\n{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    id, item.account, item.password, item.site_url, site_name, note, is_dead
                );

                // eprintln!("res: {}", res);

                response.push_str(&res);
            }
            response
        }
        Ok(ProOk::DeadLink(list)) => {
            let mut response = "2".to_string();
            for item in list {
                let is_dead = if item.1 { "0" } else { "1" };
                response.push_str(&format!("\n{}\t{}", item.0, is_dead));
            }
            response
        }
        Err(ProError::IdentityError(e)) => {
            eprintln!("IdentityError: {}", e);
            "3".to_string()
        }
        Err(ProError::DbError(e)) => {
            eprintln!("DbError: {}", e);
            "4".to_string()
        }
    };

    let response = response.as_bytes();
    // Send the response
    socket.writable().await?;
    match socket.try_write(response) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to write response: {}", e);
            Err(e)
        }
    }
}
