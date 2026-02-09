// File: src\http.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-08
// Description: 
// License: MIT

use crate::error::Result;
use crate::utils::{build_user_agent, cache_buster};
use reqwest::blocking::{Client, Response};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    secure: bool,
}

impl HttpClient {
    pub fn new(timeout: u64, secure: bool, source_address: Option<String>) -> Result<Self> {
        let builder = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .user_agent(build_user_agent())
            .gzip(true);

        // If source address is provided, bind to it
        if let Some(_addr) = source_address {
            // Note: reqwest doesn't directly support source address binding
            // This would require lower-level socket manipulation
            eprintln!("Warning: Source address binding not fully supported in this implementation");
        }

        let client = builder.build()?;

        Ok(Self { client, secure })
    }

    pub fn get(&self, url: &str) -> Result<Response> {
        let final_url = self.build_url(url)?;
        let response = self.client.get(&final_url).send()?;
        Ok(response)
    }

    pub fn post(&self, url: &str, body: Vec<u8>) -> Result<Response> {
        let final_url = self.build_url(url)?;
        let response = self
            .client
            .post(&final_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Cache-Control", "no-cache")
            .body(body)
            .send()?;
        Ok(response)
    }

    pub fn get_text(&self, url: &str) -> Result<String> {
        let response = self.get(url)?;
        Ok(response.text()?)
    }

    pub fn get_bytes(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.get(url)?;
        Ok(response.bytes()?.to_vec())
    }

    // fn build_url(&self, url: &str) -> Result<String> {
    //     let scheme = if url.starts_with(':') {
    //         if self.secure {
    //             "https"
    //         } else {
    //             "http"
    //         }
    //     } else {
    //         return Ok(url.to_string());
    //     };

    //     let url_without_colon = url.trim_start_matches(':');
    //     let delimiter = if url_without_colon.contains('?') {
    //         "&"
    //     } else {
    //         "?"
    //     };

    //     Ok(format!(
    //         "{}{}{}x={}",
    //         scheme,
    //         url_without_colon,
    //         delimiter,
    //         cache_buster()
    //     ))
    // }
    // fn build_url(&self, url: &str) -> Result<String> {
    //     if url.starts_with(':') {
    //         let scheme = if self.secure { "https" } else { "http" };
    //         let url_without_colon = url.trim_start_matches(':');
    //         let delimiter = if url_without_colon.contains('?') {
    //             "&"
    //         } else {
    //             "?"
    //         };
    //         // FIX: Added "://" after scheme + trim leading slashes
    //         Ok(format!(
    //             "{}://{}{}x={}",
    //             scheme,
    //             url_without_colon.trim_start_matches('/'),
    //             delimiter,
    //             cache_buster()
    //         ))
    //     } else {
    //         Ok(url.to_string())
    //     }
    // }

    // fn build_url(&self, url: &str) -> Result<String> {
    //     if url.starts_with("://") {
    //         let scheme = if self.secure { "https" } else { "http" };
    //         let rest = &url[3..]; // Skip "://"
    //         let delimiter = if rest.contains('?') {
    //             "&"
    //         } else {
    //             "?"
    //         };
    //         Ok(format!(
    //             "{}://{}{}x={}",
    //             scheme,
    //             rest,
    //             delimiter,
    //             cache_buster()
    //         ))
    //     } else {
    //         Ok(url.to_string())
    //     }
    // }

    fn build_url(&self, url: &str) -> Result<String> {
        if url.starts_with("://") {
            let scheme = if self.secure { "https" } else { "http" };
            let rest = &url[3..];
            let delimiter = if rest.contains('?') { "&" } else { "?" };
            Ok(format!("{}://{}{}x={}", scheme, rest, delimiter, cache_buster()))
        } else {
            Ok(url.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let client = HttpClient::new(10, false, None).unwrap();
        
        // Test with colon prefix
        let url = client.build_url("://example.com/test").unwrap();
        assert!(url.starts_with("http://example.com/test?x="));
        
        // Test with secure
        let secure_client = HttpClient::new(10, true, None).unwrap();
        let url = secure_client.build_url("://example.com/test").unwrap();
        assert!(url.starts_with("https://example.com/test?x="));
        
        // Test with existing query
        let url = client.build_url("://example.com/test?foo=bar").unwrap();
        assert!(url.contains("&x="));
    }
}
