// File: src\error.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-09
// Description: 
// License: MIT

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpeedtestError {
    #[error("Configuration retrieval error: {0}")]
    ConfigRetrieval(String),

    #[error("Servers retrieval error: {0}")]
    ServersRetrieval(String),

    #[error("Invalid server ID type: {0}")]
    InvalidServerIdType(String),

    #[error("No matched servers found")]
    NoMatchedServers,

    #[error("Speedtest mini server connection failure: {0}")]
    MiniConnectFailure(String),

    #[error("Invalid speedtest mini server: {0}")]
    InvalidMiniServer(String),

    #[error("Share results connection failure: {0}")]
    ShareResultsConnectFailure(String),

    #[error("Share results submit failure: {0}")]
    ShareResultsSubmitFailure(String),

    #[error("Upload timeout")]
    UploadTimeout,

    #[error("Best server failure: {0}")]
    BestServerFailure(String),

    #[error("Missing best server")]
    MissingBestServer,

    #[error("CLI error: {0}")]
    CliError(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("XML parse error: {0}")]
    XmlParse(#[from] quick_xml::DeError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, SpeedtestError>;
