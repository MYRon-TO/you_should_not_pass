use reqwest::Client;
use tokio::task;

use crate::db::models::{WebsiteAccount, WebsiteAccountWithDeadLink};

/// Check the dead link
/// url_list: Vec<(url, id)>
/// return: Vec<(id, is_dead_link)>
pub async fn check_dead_link(url_list: Vec<(String, i32)>) -> Vec<(i32, bool)> {
    let client = Client::new();
    let mut tasks = vec![];

    for (url, id) in url_list {
        let client = client.clone();
        tasks.push(task::spawn(async move {
            let response = client.get(&url).send().await;
            match response {
                Ok(response) => (id, !response.status().is_success()),
                Err(_) => (id, true),
            }
        }));
    }

    let mut link_status_list = vec![];
    for task in tasks {
        let result = task.await.unwrap();
        link_status_list.push(result);
    }

    link_status_list
}

pub async fn check_dead_link_info(
    url_list: Vec<WebsiteAccount>,
) -> Vec<WebsiteAccountWithDeadLink> {
    let client = Client::new();
    let mut tasks = vec![];

    for account in url_list {
        let client = client.clone();
        tasks.push(task::spawn(async move {
            let response = client.get(&account.site_url).send().await;
            match response {
                Ok(response) => {
                    eprintln!("Success: {}", account.site_url);
                    WebsiteAccountWithDeadLink {
                        id: account.id,
                        account: account.account,
                        password: account.password,
                        site_url: account.site_url,
                        site_name: account.site_name,
                        note: account.note,
                        dead_link: !response.status().is_success(),
                    }
                }
                Err(_) => {
                    eprintln!("Error: {}", account.site_url);
                    WebsiteAccountWithDeadLink {
                        id: account.id,
                        account: account.account,
                        password: account.password,
                        site_url: account.site_url,
                        site_name: account.site_name,
                        note: account.note,
                        dead_link: true,
                    }
                }
            }
        }));
    }

    let mut link_status_list = vec![];
    for task in tasks {
        let result = task.await.unwrap();
        link_status_list.push(result);
    }

    link_status_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_dead_link() {
        let url_list = vec![
            ("https://www.baidu.com".to_string(), 1),
            ("https://www.not_exist.not_exist".to_string(), 2),
        ];
        let result = check_dead_link(url_list).await;
        assert_eq!(result.len(), 2);
        for (id, is_dead_link) in result {
            if id == 1 {
                assert!(!is_dead_link);
            } else if id == 2 {
                assert!(is_dead_link);
            } else {
                panic!("unexpected id: {}", id);
            }
        }
    }

    #[tokio::test]
    async fn test_check_dead_link_info() {
        let mut list = vec![];
        let account1 = WebsiteAccount {
            id: Some(1),
            account: "test_account".to_string(),
            password: "test_password".to_string(),
            site_url: "https://www.baidu.com".to_string(),
            site_name: Some("baidu".to_string()),
            note: Some("nothing".to_string()),
        };
        let account2 = WebsiteAccount {
            id: Some(2),
            account: "test_account".to_string(),
            password: "test_password".to_string(),
            site_url: "https://www.baidu.com".to_string(),
            site_name: Some("baidu".to_string()),
            note: Some("nothing".to_string()),
        };
        let account3 = WebsiteAccount {
            id: Some(3),
            account: "test_account".to_string(),
            password: "test_password".to_string(),
            site_url: "https://www.not_exist.not_exist".to_string(),
            site_name: Some("baidu".to_string()),
            note: Some("nothing".to_string()),
        };
        list.push(account1);
        list.push(account2);
        list.push(account3);

        let result = check_dead_link_info(list).await;

        assert_eq!(result.len(), 3);
        for item in result {
            match item.id {
                Some(1) | Some(2) => {
                    assert!(!item.dead_link);
                }
                Some(3) => {
                    assert!(item.dead_link);
                }
                _ => {
                    panic!("unexpected id: {:?}", item.id);
                }
            }
        }
    }
}
