use speedtest::{Result, Speedtest};
use std::io::{self, Write};

fn main() -> Result<()> {
    println!("Speedtest Example - With Progress Callbacks\n");

    let mut speedtest = Speedtest::new(10, None, false)?;

    println!("Retrieving configuration...");
    speedtest.get_config()?;

    let config = speedtest.get_config_ref().unwrap();
    println!(
        "Testing from {} ({})\n",
        config.client.isp, config.client.ip
    );

    println!("Finding best server...");
    speedtest.determine_best_server(None)?;

    let best = speedtest.get_best_server().unwrap();
    println!(
        "Testing against: {} ({}) - {:.2} ms\n",
        best.sponsor, best.name, best.latency
    );

    // Download test with progress
    println!("Testing download speed:");
    let download = speedtest.test_download(Some(|current, total| {
        print!("\rProgress: {}/{} requests", current, total);
        io::stdout().flush().unwrap();
    }))?;
    println!("\nDownload: {:.2} Mbps\n", download / 1_000_000.0);

    // Upload test with progress
    println!("Testing upload speed:");
    let upload = speedtest.test_upload(
        Some(|current, total| {
            print!("\rProgress: {}/{} requests", current, total);
            io::stdout().flush().unwrap();
        }),
        true,
    )?;
    println!("\nUpload: {:.2} Mbps\n", upload / 1_000_000.0);

    // Get and display results
    let results = speedtest.get_results();
    println!("Results:");
    println!("  Ping: {:.2} ms", results.ping);
    println!("  Download: {:.2} Mbps", results.download / 1_000_000.0);
    println!("  Upload: {:.2} Mbps", results.upload / 1_000_000.0);

    // Try to share results
    println!("\nSharing results...");
    match speedtest.share_results() {
        Ok(url) => println!("Share URL: {}", url),
        Err(e) => println!("Failed to share: {}", e),
    }

    Ok(())
}
