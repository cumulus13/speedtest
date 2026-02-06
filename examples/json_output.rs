use speedtest::{Result, Speedtest};

fn main() -> Result<()> {
    let mut speedtest = Speedtest::new(10, None, false)?;

    // Get configuration
    speedtest.get_config()?;

    // Find best server
    speedtest.determine_best_server(None)?;

    // Run tests
    speedtest.test_download(None::<fn(usize, usize)>)?;
    speedtest.test_upload(None::<fn(usize, usize)>, true)?;

    // Get results and output as JSON
    let results = speedtest.get_results();
    let json = serde_json::to_string_pretty(results)?;
    println!("{}", json);

    Ok(())
}
