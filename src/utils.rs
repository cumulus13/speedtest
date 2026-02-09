// File: src\utils.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-09
// Description: 
// License: MIT

/// Calculate distance between two geographic coordinates using Haversine formula
pub fn distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let radius = 6371.0; // Earth radius in km

    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    radius * c
}

/// Build user agent string
pub fn build_user_agent() -> String {
    format!(
        "Mozilla/5.0 ({} {}) speedtest/1.0.0",
        std::env::consts::OS,
        std::env::consts::ARCH
    )
}

/// Generate cache buster parameter
pub fn cache_buster() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        // Test distance between New York and London
        let d = distance(40.7128, -74.0060, 51.5074, -0.1278);
        // Should be approximately 5570 km
        assert!((d - 5570.0).abs() < 50.0);
    }

    #[test]
    fn test_user_agent() {
        let ua = build_user_agent();
        assert!(ua.contains("Mozilla/5.0"));
        assert!(ua.contains("speedtest"));
    }

    #[test]
    fn test_cache_buster() {
        let cb1 = cache_buster();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let cb2 = cache_buster();
        assert_ne!(cb1, cb2);
    }
}
