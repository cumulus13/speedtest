// File: src\models.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-09
// Description: 
// License: MIT

use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub lat: String,
    #[serde(default)]
    pub lon: String,
    #[serde(default)]
    pub isp: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub isprating: String,
    #[serde(default)]
    pub rating: String,
    #[serde(default)]
    pub ispdlavg: String,
    #[serde(default)]
    pub ispulavg: String,
    #[serde(default)]
    pub loggedin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: u32,
    pub sponsor: String,
    pub name: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub url: String,
    #[serde(default)]
    pub d: f64,
    #[serde(default)]
    pub latency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub client: Client,
    pub ignore_servers: Vec<u32>,
    pub sizes: Sizes,
    pub counts: Counts,
    pub threads: Threads,
    pub length: Length,
    pub upload_max: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sizes {
    pub upload: Vec<usize>,
    pub download: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counts {
    pub upload: usize,
    pub download: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threads {
    pub upload: usize,
    pub download: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Length {
    pub upload: u64,
    pub download: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedtestResults {
    pub download: f64,
    pub upload: f64,
    pub ping: f64,
    pub server: Server,
    pub timestamp: String,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
    pub client: Client,
}

impl SpeedtestResults {
    pub fn new(client: Client, server: Server) -> Self {
        Self {
            download: 0.0,
            upload: 0.0,
            ping: 0.0,
            server,
            timestamp: chrono::Utc::now().to_rfc3339(),
            bytes_received: 0,
            bytes_sent: 0,
            share: None,
            client,
        }
    }

    pub fn to_csv(&self, delimiter: char) -> crate::error::Result<String> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter as u8)
            .from_writer(vec![]);

        wtr.write_record(&[
            self.server.id.to_string(),
            self.server.sponsor.clone(),
            self.server.name.clone(),
            self.timestamp.clone(),
            format!("{:.2}", self.server.d),
            format!("{:.3}", self.ping),
            format!("{:.2}", self.download),
            format!("{:.2}", self.upload),
            self.share.clone().unwrap_or_default(),
            self.client.ip.clone(),
        ])?;

        let inner = wtr.into_inner().map_err(|e| {
            crate::error::SpeedtestError::Csv(csv::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to finalize CSV: {}", e)
            )))
        })?;
        Ok(String::from_utf8_lossy(&inner).to_string())
    }

    pub fn csv_header(delimiter: char) -> crate::error::Result<String> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter as u8)
            .from_writer(vec![]);

        wtr.write_record(&[
            "Server ID",
            "Sponsor",
            "Server Name",
            "Timestamp",
            "Distance",
            "Ping",
            "Download",
            "Upload",
            "Share",
            "IP Address",
        ])?;

        let inner = wtr.into_inner().map_err(|e| {
            crate::error::SpeedtestError::Csv(csv::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to finalize CSV: {}", e)
            )))
        })?;
        Ok(String::from_utf8_lossy(&inner).to_string())
    }

    pub fn to_json(&self, pretty: bool) -> crate::error::Result<String> {
        if pretty {
            Ok(serde_json::to_string_pretty(self)?)
        } else {
            Ok(serde_json::to_string(self)?)
        }
    }
}