mod action;
mod error;
mod check_dead_link;
mod auth;

use crate::db::Db;
use action::*;
use check_dead_link::check_dead_link;
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

    let _ = handle_action(action, &socket, db).await;
}

async fn handle_action(
    action: Action,
    socket: &TcpStream,
    db: Arc<Db>,
) -> Result<(), error::Error> {
    match action {
        Action::CheckIdentity { password } => {
            // Check the password
            if let Err(e) = auth::authourize(password).await{
                return Err(error::Error::IdentityError(e));
            }

            Ok(())
        }
        Action::GetInfo => {
            // GetInfo
            if let Err(e) = db.get_all_website_account().await{
                return Err(error::Error::DbError(e));
            }
            Ok(())
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
                return Err(error::Error::DbError(e));
            }
            Ok(())
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
                return Err(error::Error::DbError(e));
            }
            Ok(())
        }
        Action::DeleteWebsiteAccount { website_id } => {
            // Delete the website account
            if let Err(e) = db.delete_website_account(website_id).await{
                return Err(error::Error::DbError(e));
            }
            Ok(())
        }
        Action::CheckDeadLink => {
            // Check the dead link
            match db.get_all_id_and_url().await{
                Ok(id_and_url) => {
                    // todo
                    let _list = check_dead_link(id_and_url).await;
                }
                Err(e) => {
                    return Err(error::Error::DbError(e));
                }
            };
            Ok(())
        }
    }
}
