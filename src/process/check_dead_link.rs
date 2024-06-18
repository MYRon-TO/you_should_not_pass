use reqwest::Client;
use tokio::task;

/// Check the dead link
/// url_list: Vec<(url, id)>
/// return: Vec<(id, is_dead_link)>
pub async fn check_dead_link(url_list: Vec<(String, i32)>) -> Vec<(i32, bool)>{
    let client = Client::new();
    let mut tasks = vec![];

    for (url, id) in url_list {
        let client = client.clone();
        tasks.push(task::spawn(async move {
            let response = client.get(&url).send().await;
            match response {
                Ok(response) => {
                    (id, !response.status().is_success())
                }
                Err(_) => {
                    (id, true)
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
}
