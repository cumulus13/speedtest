use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use speedtest::{Result, Speedtest, SpeedtestError, SpeedtestResults};
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "speedtest",
    version,
    about = "Command line interface for testing internet bandwidth using speedtest.net",
    long_about = None
)]
struct Args {
    /// Do not perform download test
    #[arg(long)]
    no_download: bool,

    /// Do not perform upload test
    #[arg(long)]
    no_upload: bool,

    /// Only use a single connection instead of multiple
    #[arg(long)]
    single: bool,

    /// Display values in bytes instead of bits
    #[arg(long)]
    bytes: bool,

    /// Generate and provide a URL to the speedtest.net share results image
    #[arg(long)]
    share: bool,

    /// Suppress verbose output, only show basic information
    #[arg(long)]
    simple: bool,

    /// Suppress verbose output, only show basic information in CSV format
    #[arg(long)]
    csv: bool,

    /// Single character delimiter to use in CSV output
    #[arg(long, default_value = ",")]
    csv_delimiter: char,

    /// Print CSV headers
    #[arg(long)]
    csv_header: bool,

    /// Suppress verbose output, only show basic information in JSON format
    #[arg(long)]
    json: bool,

    /// Display a list of speedtest.net servers sorted by distance
    #[arg(long)]
    list: bool,

    /// Specify a server ID to test against (can be supplied multiple times)
    #[arg(long)]
    server: Option<Vec<u32>>,

    /// Exclude a server from selection (can be supplied multiple times)
    #[arg(long)]
    exclude: Option<Vec<u32>>,

    /// Source IP address to bind to
    #[arg(long)]
    source: Option<String>,

    /// HTTP timeout in seconds
    #[arg(long, default_value = "10")]
    timeout: u64,

    /// Use HTTPS instead of HTTP
    #[arg(long)]
    secure: bool,

    /// Do not pre-allocate upload data
    #[arg(long)]
    no_pre_allocate: bool,

    /// Enable debug output
    #[arg(long, hide = true)]
    debug: bool,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Handle CSV header
    if args.csv_header {
        println!("{}", SpeedtestResults::csv_header(args.csv_delimiter));
        return Ok(());
    }

    // Validate arguments
    if args.no_download && args.no_upload {
        return Err(SpeedtestError::CliError(
            "Cannot supply both --no-download and --no-upload".to_string(),
        ));
    }

    let quiet = args.simple || args.csv || args.json;
    let (units, divisor) = if args.bytes {
        ("byte", 8.0)
    } else {
        ("bit", 1.0)
    };

    // Create speedtest instance
    if !quiet {
        println!("Retrieving speedtest.net configuration...");
    }

    let mut speedtest = Speedtest::new(args.timeout, args.source, args.secure)?;

    // Get configuration
    speedtest.get_config()?;

    let config = speedtest.get_config_ref().unwrap();
    if !quiet {
        println!(
            "Testing from {} ({})...",
            config.client.isp, config.client.ip
        );
    }

    // Handle server list
    if args.list {
        if !quiet {
            println!("Retrieving speedtest.net server list...");
        }
        speedtest.get_servers(None, None)?;

        // Display servers sorted by distance
        let mut all_servers = Vec::new();
        for servers in speedtest.get_servers(None, None)?.values() {
            all_servers.extend(servers.iter().cloned());
        }
        all_servers.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

        for server in all_servers {
            println!(
                "{:5}) {} ({}, {}) [{:.2} km]",
                server.id, server.sponsor, server.name, server.country, server.d
            );
        }
        return Ok(());
    }

    // Get servers and find best
    if !quiet {
        println!("Retrieving speedtest.net server list...");
    }

    speedtest.get_servers(args.server.clone(), args.exclude)?;

    if args.server.is_some() && args.server.as_ref().unwrap().len() == 1 {
        if !quiet {
            println!("Retrieving information for the selected server...");
        }
    } else if !quiet {
        println!("Selecting best server based on ping...");
    }

    let spinner = if !quiet {
        let sp = ProgressBar::new_spinner();
        sp.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        sp.set_message("Testing latency...");
        sp.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(sp)
    } else {
        None
    };

    speedtest.determine_best_server(None)?;

    if let Some(sp) = spinner {
        sp.finish_and_clear();
    }

    let best = speedtest.get_best_server().unwrap();
    if !quiet {
        println!(
            "Hosted by {} ({}) [{:.2} km]: {:.2} ms",
            best.sponsor, best.name, best.d, best.latency
        );
    }

    // Download test
    if !args.no_download {
        if !quiet {
            println!("Testing download speed...");
        }

        let pb = if !quiet {
            let progress = ProgressBar::new(100);
            progress.set_style(
                ProgressStyle::default_bar()
                    .template("[{bar:40.cyan/blue}] {pos}/{len}")
                    .unwrap()
                    .progress_chars("#>-"),
            );
            Some(progress)
        } else {
            None
        };

        let pb_clone = pb.clone();
        speedtest.test_download(Some(move |current, total| {
            if let Some(ref p) = pb_clone {
                p.set_length(total as u64);
                p.set_position(current as u64);
            }
        }))?;

        if let Some(p) = pb {
            p.finish_and_clear();
        }

        let results = speedtest.get_results();
        if !quiet {
            println!(
                "Download: {:.2} M{}/s",
                (results.download / 1_000_000.0) / divisor,
                units
            );
        }
    } else if !quiet {
        println!("Skipping download test");
    }

    // Upload test
    if !args.no_upload {
        if !quiet {
            println!("Testing upload speed...");
        }

        let pb = if !quiet {
            let progress = ProgressBar::new(100);
            progress.set_style(
                ProgressStyle::default_bar()
                    .template("[{bar:40.cyan/blue}] {pos}/{len}")
                    .unwrap()
                    .progress_chars("#>-"),
            );
            Some(progress)
        } else {
            None
        };

        let pb_clone = pb.clone();
        speedtest.test_upload(
            Some(move |current, total| {
                if let Some(ref p) = pb_clone {
                    p.set_length(total as u64);
                    p.set_position(current as u64);
                }
            }),
            !args.no_pre_allocate,
        )?;

        if let Some(p) = pb {
            p.finish_and_clear();
        }

        let results = speedtest.get_results();
        if !quiet {
            println!(
                "Upload: {:.2} M{}/s",
                (results.upload / 1_000_000.0) / divisor,
                units
            );
        }
    } else if !quiet {
        println!("Skipping upload test");
    }

    // Share results
    if args.share && !args.simple {
        if !quiet {
            println!("Sharing results...");
        }
        match speedtest.share_results() {
            Ok(url) => {
                if !quiet && !args.csv && !args.json {
                    println!("Share results: {}", url);
                }
            }
            Err(e) => {
                if !quiet {
                    eprintln!("Warning: Failed to share results: {}", e);
                }
            }
        }
    }

    // Output results
    let results = speedtest.get_results();

    if args.simple {
        println!("{}", results.to_simple(units, divisor));
    } else if args.csv {
        println!("{}", results.to_csv(args.csv_delimiter));
    } else if args.json {
        println!("{}", serde_json::to_string_pretty(results)?);
    }

    Ok(())
}
