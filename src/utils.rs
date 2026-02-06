/// Calculate distance between two coordinates using Haversine formula
pub fn distance(origin: (f64, f64), destination: (f64, f64)) -> f64 {
    let (lat1, lon1) = origin;
    let (lat2, lon2) = destination;
    let radius = 6371.0; // Earth radius in km

    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    radius * c
}

/// Build a User-Agent string
pub fn build_user_agent() -> String {
    format!(
        "Mozilla/5.0 ({}) speedtest-rs/{}",
        std::env::consts::OS,
        env!("CARGO_PKG_VERSION")
    )
}

/// Generate cache-busting URL parameter
pub fn cache_bust() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("x={}", timestamp)
}

/// Add query parameter to URL
pub fn add_query_param(url: &str, param: &str) -> String {
    let delimiter = if url.contains('?') { "&" } else { "?" };
    format!("{}{}{}", url, delimiter, param)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        // Distance between New York and London (approx 5570 km)
        let ny = (40.7128, -74.0060);
        let london = (51.5074, -0.1278);
        let dist = distance(ny, london);
        assert!((dist - 5570.0).abs() < 100.0);
    }

    #[test]
    fn test_add_query_param() {
        assert_eq!(
            add_query_param("http://example.com", "foo=bar"),
            "http://example.com?foo=bar"
        );
        assert_eq!(
            add_query_param("http://example.com?baz=qux", "foo=bar"),
            "http://example.com?baz=qux&foo=bar"
        );
    }
}