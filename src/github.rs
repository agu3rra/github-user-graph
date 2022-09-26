//! mod `github.rs`
//! 
//! This module implements interactions with Github API
use std::{self, env, time::Duration};
use reqwest::{self, header::HeaderMap, Response, Client};
use tokio::runtime::Runtime;

/// User representation we care about
pub struct User {
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
}

impl Github {
    pub fn new(base_url: String, token: String, timeout: Duration) -> Github {
        let client: Client = reqwest::Client::builder()
            .timeout(timeout)
            .build().unwrap();
        Github {
            base_url: base_url,
            token: token,
            timeout: timeout,
            client: client,
        }
    }

    /// Creates authorization headers for HTTP requests.
    fn authorization_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        headers.insert("Authorization", format!("Bearer {}", self.token).parse().unwrap());
        headers
    }

    /// Obtains relevant user details for us
    pub fn get_user(&self, user: String) -> Result<User, std::io::Error> {
        let endpoint = format!("{}/api/v3/user", self.base_url);

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_user() {
        let token = env::var("GITHUB_TOKEN")
            .expect("Test requires GITHUB_TOKEN set as an ENV to work.");
        let gh: Github = Github::new(
            String::from("https://github.com"),
            token,
            Duration::new(15, 0),
        );
        let test_user = env::var("GITHUB_USER")
            .expect("GITHUB_USER must be set as an ENV for the test to work.");
        let user: User = gh.get_user(test_user.to_owned()).unwrap();
        assert!(user.login == test_user);
    }
}