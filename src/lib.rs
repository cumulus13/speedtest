// File: src\lib.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-09
// Description: 
// License: MIT

pub mod error;
pub mod http;
pub mod models;
pub mod speedtest;
pub mod utils;

pub use error::{Result, SpeedtestError};
pub use models::*;
pub use speedtest::Speedtest;
