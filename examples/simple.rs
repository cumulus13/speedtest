use speedtest::{Result, Speedtest};

fn main() -> Result<()> {
    println!("Speedtest Example - Simple Usage\n");

    // Create a new speedtest instance
    let mut speedtest = Speedtest::new(10, None, false)?;

    // Get configuration
    println!("Retrieving configuration...");
    speedtest.get_config()?;

    let config = speedtest.get_config_ref().unwrap();
    println!(
        "Testing from {} ({})\n",
        config.client.isp, config.client.ip
    );

    // Find best server
    println!("Finding best server...");
    speedtest.determine_best_server(None)?;

    let best = speedtest.get_best_server().unwrap();
    println!(
        "Testing against: {} ({}) [{:.2} km] - {:.2} ms\n",
        best.sponsor, best.name, best.d, best.latency
    );

    // Download test
    println!("Testing download speed...");
    let download = speedtest.test_download(None::<fn(usize, usize)>)?;
    println!("Download: {:.2} Mbps\n", download / 1_000_000.0);

    // Upload test
    println!("Testing upload speed...");
    let upload = speedtest.test_upload(None::<fn(usize, usize)>, true)?;
    println!("Upload: {:.2} Mbps\n", upload / 1_000_000.0);

    // Display results
    let results = speedtest.get_results();
    println!("Final Results:");
    println!("  Ping: {:.2} ms", results.ping);
    println!("  Download: {:.2} Mbps", results.download / 1_000_000.0);
    println!("  Upload: {:.2} Mbps", results.upload / 1_000_000.0);
    println!("  Bytes Received: {}", results.bytes_received);
    println!("  Bytes Sent: {}", results.bytes_sent);

    Ok(())
}
