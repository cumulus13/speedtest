//! # Speedtest
//!
//! A robust Rust implementation of speedtest-cli for testing internet bandwidth.
//!
//! This library provides functionality to test your internet connection speed
//! by connecting to speedtest.net servers.
//!
//! ## Example
//!
//! ```no_run
//! use speedtest::Speedtest;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut speedtest = Speedtest::new(10, None, false)?;
//!     
//!     // Get configuration
//!     speedtest.get_config()?;
//!     
//!     // Find best server
//!     speedtest.determine_best_server(None)?;
//!     
//!     // Run tests
//!     let download_speed = speedtest.test_download(None::<fn(usize, usize)>)?;
//!     let upload_speed = speedtest.test_upload(None::<fn(usize, usize)>, true)?;
//!     
//!     println!("Download: {:.2} Mbps", download_speed / 1_000_000.0);
//!     println!("Upload: {:.2} Mbps", upload_speed / 1_000_000.0);
//!     
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod http;
pub mod types;
pub mod utils;

mod speedtest;
mod latency;
mod download;
mod upload;
mod share;

pub use error::{Result, SpeedtestError};
pub use speedtest::Speedtest;
pub use types::{
    Client, Config, Counts, Length, Server, Sizes, SpeedtestResults, Threads,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_speedtest() {
        let speedtest = Speedtest::new(10, None, false);
        assert!(speedtest.is_ok());
    }
}
