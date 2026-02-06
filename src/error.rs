use thiserror::Error;

/// Custom error types for speedtest operations
#[derive(Error, Debug)]
pub enum SpeedtestError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Configuration retrieval failed: {0}")]
    ConfigRetrievalError(String),

    #[error("Server retrieval failed: {0}")]
    ServersRetrievalError(String),

    #[error("Invalid speedtest configuration: {0}")]
    ConfigError(String),

    #[error("Invalid servers XML: {0}")]
    ServersError(String),

    #[error("Invalid server ID type: {0}")]
    InvalidServerIdType(String),

    #[error("No matched servers found")]
    NoMatchedServers,

    #[error("Could not connect to speedtest mini server: {0}")]
    MiniConnectFailure(String),

    #[error("Invalid speedtest mini server: {0}")]
    InvalidMiniServer(String),

    #[error("Share results connection failed: {0}")]
    ShareResultsConnectFailure(String),

    #[error("Share results submit failed: {0}")]
    ShareResultsSubmitFailure(String),

    #[error("Upload timeout reached")]
    UploadTimeout,

    #[error("Unable to determine best server")]
    BestServerFailure,

    #[error("Best server not determined - call get_best_server first")]
    MissingBestServer,

    #[error("CLI error: {0}")]
    CliError(String),

    #[error("XML parsing error: {0}")]
    XmlError(#[from] quick_xml::DeError),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid location: lat={lat:?}, lon={lon:?}")]
    InvalidLocation { lat: Option<String>, lon: Option<String> },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, SpeedtestError>;
