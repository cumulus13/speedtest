// File: examples\debug-xml.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-09
// Description: Debug program to check XML response from speedtest.net
// License: MIT

// Debug program to check XML response from speedtest.net
// Run with: cargo run --bin debug-xml

use reqwest::blocking::Client;

fn main() {
    println!("Fetching speedtest.net configuration XML...\n");
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Mozilla/5.0 speedtest/1.0.0")
        .build()
        .expect("Failed to create HTTP client");
    
    match client.get("http://www.speedtest.net/speedtest-config.php").send() {
        Ok(response) => {
            println!("Status: {}", response.status());
            match response.text() {
                Ok(xml) => {
                    println!("\nFull XML Response:");
                    println!("{}", "=".repeat(80));
                    println!("{}", xml);
                    println!("{}", "=".repeat(80));
                    
                    // Try to parse and show structure
                    println!("\nAttempting to parse...");
                    if let Err(e) = quick_xml::de::from_str::<serde_json::Value>(&xml) {
                        println!("Parse error: {}", e);
                    }
                }
                Err(e) => {
                    println!("Failed to read response text: {}", e);
                }
            }
        }
        Err(e) => {
            println!("HTTP request failed: {}", e);
        }
    }
}
