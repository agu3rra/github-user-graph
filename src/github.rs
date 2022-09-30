//! mod `github.rs`
//! 
//! This module implements interactions with Github API
use std::{self, time::Duration};
use tokio::runtime::Runtime;
use reqwest::{self, header::HeaderMap, Response, Client};
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// User representation we care about
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    login: String,
    followers_url: String,
    following_url: String,
}

/// A user and its relationships with other users
pub struct UserRelations {
    login: String,
    followers: Vec<String>,
    following: Vec<String>,
}

/// Github struct
pub struct Github {
    pub base_url: String,
    pub token: String,
    pub timeout: Duration,
    client: reqwest::Client,
    runtime: Runtime,
    headers: HeaderMap,
}

impl Github {
    pub fn new(base_url: String, token: String, timeout: Duration) -> Github {
        let client: Client = reqwest::Client::builder()
            .timeout(timeout)
            .build().unwrap();
        let runtime = Runtime::new()
            .expect("Unable to create tokio runtime.");
        let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
            headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
            headers.insert("User-Agent", "github-user-graph-in-rust".parse().unwrap());
        Github {
            base_url: base_url,
            token: token,
            timeout: timeout,
            client: client,
            runtime: runtime,
            headers: headers,
        }
    }

    /// Obtains relevant user details for us
    pub fn get_user(&self, user: String) -> Result<User, std::io::Error> {
        let endpoint = format!("{}/user", self.base_url);
        self.runtime.block_on( async {
            let resp: Response = self.client
                .get(endpoint.to_owned())
                .headers(self.headers.to_owned())
                .send()
                .await.expect("Error sending HTTP request");
            if resp.status() != reqwest::StatusCode::OK {
                panic!("Error response: {}", resp.status());
            }
            let user_data: Value = resp.json().await
                .expect("Unable to parse JSON response.");
            let user = User {
                login: user_data.get("login")
                    .unwrap()
                    .to_string()
                    .replace("\"", ""),
                followers_url: user_data.get("followers_url")
                    .unwrap()
                    .to_string()
                    .replace("\"",""),
                following_url: user_data.get("followers_url")
                    .unwrap()
                    .to_string()
                    .replace("\"", ""),
            };
            return Ok(user);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_user() {
        let token = std::env::var("GITHUB_TOKEN")
            .expect("Test requires GITHUB_TOKEN set as an ENV to work.");
        let gh: Github = Github::new(
            String::from("https://api.github.com"),
            token,
            Duration::new(15, 0),
        );
        let test_user = std::env::var("GITHUB_USER")
            .expect("GITHUB_USER must be set as an ENV for the test to work.");
        let user: User = gh.get_user(test_user.to_owned()).unwrap();
        assert!(user.login == test_user);
    }
}