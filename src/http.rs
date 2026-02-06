use crate::error::{Result, SpeedtestError};
use crate::utils;
use reqwest::blocking::{Client, ClientBuilder, Response};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    secure: bool,
}

impl HttpClient {
    pub fn new(timeout: u64, source_address: Option<String>, secure: bool) -> Result<Self> {
        let mut builder = ClientBuilder::new()
            .timeout(Duration::from_secs(timeout))
            .user_agent(utils::build_user_agent())
            .gzip(true)
            .no_proxy();

        // Note: reqwest doesn't support binding to specific source address easily
        // This would require custom TLS configuration or OS-level routing

        let client = builder.build()?;

        Ok(Self { client, secure })
    }

    pub fn get(&self, url: &str) -> Result<Response> {
        let url = self.build_url(url);
        let url = utils::add_query_param(&url, &utils::cache_bust());

        self.client
            .get(&url)
            .header("Cache-Control", "no-cache")
            .send()
            .map_err(|e| e.into())
    }

    pub fn post(&self, url: &str, data: Vec<u8>) -> Result<Response> {
        let url = self.build_url(url);
        let url = utils::add_query_param(&url, &utils::cache_bust());

        self.client
            .post(&url)
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(data)
            .send()
            .map_err(|e| e.into())
    }

    pub fn get_text(&self, url: &str) -> Result<String> {
        Ok(self.get(url)?.text()?)
    }

    pub fn get_bytes(&self, url: &str) -> Result<Vec<u8>> {
        Ok(self.get(url)?.bytes()?.to_vec())
    }

    fn build_url(&self, url: &str) -> String {
        if url.starts_with(':') {
            let scheme = if self.secure { "https" } else { "http" };
            format!("{}{}", scheme, url)
        } else if !url.starts_with("http://") && !url.starts_with("https://") {
            let scheme = if self.secure { "https" } else { "http" };
            format!("{}://{}", scheme, url)
        } else {
            url.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let client = HttpClient::new(10, None, false).unwrap();
        assert_eq!(
            client.build_url("://example.com/test"),
            "http://example.com/test"
        );

        let secure_client = HttpClient::new(10, None, true).unwrap();
        assert_eq!(
            secure_client.build_url("://example.com/test"),
            "https://example.com/test"
        );
    }
}
