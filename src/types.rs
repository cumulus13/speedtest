use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Client information from speedtest configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub ip: String,
    pub lat: String,
    pub lon: String,
    pub isp: String,
    #[serde(rename = "isprating")]
    pub isp_rating: Option<String>,
    #[serde(rename = "ispdlavg")]
    pub isp_dl_avg: Option<String>,
    #[serde(rename = "ispulavg")]
    pub isp_ul_avg: Option<String>,
    pub country: Option<String>,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: u32,
    pub sponsor: String,
    pub name: String,
    pub country: String,
    #[serde(rename = "cc")]
    pub country_code: String,
    pub host: String,
    pub url: String,
    pub lat: String,
    pub lon: String,
    #[serde(skip)]
    pub d: f64, // distance in km
    #[serde(skip)]
    pub latency: f64, // latency in ms
}

/// Speedtest configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub client: Client,
    pub ignore_servers: Vec<u32>,
    pub sizes: Sizes,
    pub counts: Counts,
    pub threads: Threads,
    pub length: Length,
    pub upload_max: usize,
}

#[derive(Debug, Clone)]
pub struct Sizes {
    pub upload: Vec<usize>,
    pub download: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Counts {
    pub upload: usize,
    pub download: usize,
}

#[derive(Debug, Clone)]
pub struct Threads {
    pub upload: usize,
    pub download: usize,
}

#[derive(Debug, Clone)]
pub struct Length {
    pub upload: u64,
    pub download: u64,
}

/// Test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedtestResults {
    pub download: f64,      // bits per second
    pub upload: f64,        // bits per second
    pub ping: f64,          // milliseconds
    pub server: ResultServer,
    pub client: Client,
    pub timestamp: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultServer {
    pub id: u32,
    pub sponsor: String,
    pub name: String,
    pub country: String,
    pub d: f64,
    pub latency: f64,
    pub url: String,
}

impl Default for SpeedtestResults {
    fn default() -> Self {
        Self {
            download: 0.0,
            upload: 0.0,
            ping: 0.0,
            server: ResultServer {
                id: 0,
                sponsor: String::new(),
                name: String::new(),
                country: String::new(),
                d: 0.0,
                latency: 0.0,
                url: String::new(),
            },
            client: Client {
                ip: String::new(),
                lat: String::new(),
                lon: String::new(),
                isp: String::new(),
                isp_rating: None,
                isp_dl_avg: None,
                isp_ul_avg: None,
                country: None,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            bytes_sent: 0,
            bytes_received: 0,
            share: None,
        }
    }
}

impl SpeedtestResults {
    pub fn csv_header(delimiter: char) -> String {
        let headers = vec![
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
        ];
        headers.join(&delimiter.to_string())
    }

    pub fn to_csv(&self, delimiter: char) -> String {
        let fields = vec![
            self.server.id.to_string(),
            self.server.sponsor.clone(),
            self.server.name.clone(),
            self.timestamp.clone(),
            format!("{:.2}", self.server.d),
            format!("{:.2}", self.ping),
            format!("{:.2}", self.download),
            format!("{:.2}", self.upload),
            self.share.clone().unwrap_or_default(),
            self.client.ip.clone(),
        ];
        fields.join(&delimiter.to_string())
    }

    pub fn to_simple(&self, units: &str, divisor: f64) -> String {
        format!(
            "Ping: {:.2} ms\nDownload: {:.2} M{}/s\nUpload: {:.2} M{}/s",
            self.ping,
            (self.download / 1000.0 / 1000.0) / divisor,
            units,
            (self.upload / 1000.0 / 1000.0) / divisor,
            units
        )
    }
}

/// XML structures for parsing speedtest config
#[derive(Debug, Deserialize)]
pub struct SpeedtestConfig {
    #[serde(rename = "client")]
    pub client: Client,
    #[serde(rename = "server-config")]
    pub server_config: ServerConfig,
    pub download: DownloadConfig,
    pub upload: UploadConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(rename = "ignoreids")]
    pub ignore_ids: String,
    #[serde(rename = "threadcount")]
    pub thread_count: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadConfig {
    #[serde(rename = "testlength")]
    pub test_length: String,
    #[serde(rename = "threadsperurl")]
    pub threads_per_url: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadConfig {
    #[serde(rename = "testlength")]
    pub test_length: String,
    #[serde(rename = "threads")]
    pub threads: String,
    #[serde(rename = "maxchunkcount")]
    pub max_chunk_count: String,
    #[serde(rename = "ratio")]
    pub ratio: String,
}

/// XML structures for parsing servers
#[derive(Debug, Deserialize)]
pub struct ServersRoot {
    #[serde(rename = "servers")]
    pub servers: ServersContainer,
}

#[derive(Debug, Deserialize)]
pub struct ServersContainer {
    #[serde(rename = "server", default)]
    pub server: Vec<ServerXml>,
}

#[derive(Debug, Deserialize)]
pub struct ServerXml {
    pub id: String,
    pub sponsor: String,
    pub name: String,
    pub country: String,
    #[serde(rename = "cc")]
    pub country_code: String,
    pub host: String,
    pub url: String,
    pub lat: String,
    pub lon: String,
}
