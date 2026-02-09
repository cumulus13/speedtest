// File: src\main.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-08
// Description: Command line interface for testing internet bandwidth using speedtest.net
// License: MIT

use clap::{Parser, ArgAction};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use speedtest::{Result, Speedtest, SpeedtestError, SpeedtestResults};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration; 
use clap_version_flag::colorful_version;

/// Command line interface for testing internet bandwidth using speedtest.net
#[derive(Parser, Debug)]
#[command(author = "Hadi Cahyadi <cumulus13@gmail.com>")]
// #[command(version = "1.0.0")]
#[command(disable_version_flag = true)]
#[command(about = "Command line interface for testing internet bandwidth using speedtest.net", long_about = "Command-line interface for testing internet bandwidth using speedtest.net, written in Rust. Providing better performance, lower memory usage, and cross-platform compatibility.")]
struct Args {
    #[arg[short = 'V', long = "version", action = ArgAction::SetTrue]]

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
    csv_delimiter: String,

    /// Print CSV headers
    #[arg(long)]
    csv_header: bool,

    /// Suppress verbose output, only show basic information in JSON format
    #[arg(long)]
    json: bool,

    /// Display a list of speedtest.net servers sorted by distance
    #[arg(long)]
    list: bool,

    /// Specify a server ID to test against (can be used multiple times)
    #[arg(long, value_name = "ID")]
    server: Vec<u32>,

    /// Exclude a server from selection (can be used multiple times)
    #[arg(long, value_name = "ID")]
    exclude: Vec<u32>,

    /// URL of the Speedtest Mini server
    #[arg(long)]
    mini: Option<String>,

    /// Source IP address to bind to
    #[arg(long)]
    source: Option<String>,

    /// HTTP timeout in seconds
    #[arg(long, default_value = "10")]
    timeout: u64,

    /// Use HTTPS instead of HTTP when communicating with speedtest.net
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
    let os_args: Vec<String> = std::env::args().collect();
    
    // Check ANY argument for -V or --version
    if os_args.iter().any(|arg| arg == "-V" || arg == "--version") {
        let version = colorful_version!();
        version.print_and_exit();
    }

    // Normal program execution
    if let Err(e) = run() {
        eprintln!("{} {}", "ERROR:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    
    let args = Args::parse();

    // Validate arguments
    if args.no_download && args.no_upload {
        return Err(SpeedtestError::CliError(
            "Cannot supply both --no-download and --no-upload".to_string(),
        ));
    }

    if args.csv_delimiter.len() != 1 {
        return Err(SpeedtestError::CliError(
            "--csv-delimiter must be a single character".to_string(),
        ));
    }

    // Handle CSV header request
    if args.csv_header {
        let delimiter = args.csv_delimiter.chars().next().unwrap();
        println!("{}", SpeedtestResults::csv_header(delimiter)?);
        return Ok(());
    }

    let quiet = args.simple || args.csv || args.json;

    // Setup signal handler for Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        eprintln!("\n{}", "Cancelling...".yellow());
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Initialize speedtest
    if !quiet {
        println!("{}", "Retrieving speedtest.net configuration...".cyan());
    }

    let mut speedtest = Speedtest::new(args.timeout, args.secure, args.source)?;
    
    let config = speedtest.get_config()?;

    if !quiet {
        println!(
            "{}",
            format!(
                "Testing from {} ({})...",
                config.client.isp, config.client.ip
            )
            .green()
        );
    }

    // Handle server list request
    if args.list {
        if !quiet {
            println!("{}", "Retrieving speedtest.net server list...".cyan());
        }
        
        speedtest.get_servers(None, None)?;
        
        let mut all_servers: Vec<_> = speedtest
            .get_closest_servers(100)?
            .to_vec();
        
        all_servers.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

        for server in all_servers {
            println!(
                "{:>5}) {} ({}, {}) [{:.2} km]",
                server.id, server.sponsor, server.name, server.country, server.d
            );
        }
        return Ok(());
    }

    // Get servers
    if args.mini.is_none() {
        if !quiet {
            println!("{}", "Retrieving speedtest.net server list...".cyan());
        }

        let server_ids = if !args.server.is_empty() {
            Some(args.server.as_slice())
        } else {
            None
        };

        let exclude = if !args.exclude.is_empty() {
            Some(args.exclude.as_slice())
        } else {
            None
        };

        speedtest.get_servers(server_ids, exclude)?;

        if !quiet {
            if args.server.len() == 1 {
                println!("{}", "Retrieving information for the selected server...".cyan());
            } else {
                println!("{}", "Selecting best server based on ping...".cyan());
            }
        }

        speedtest.get_best_server(None)?;
    } else {
        return Err(SpeedtestError::CliError(
            "Mini server support not yet implemented".to_string(),
        ));
    }

    // let best_server = speedtest.get_best_server(None)?.clone();

    // if !quiet {
    //     println!(
    //         "{}",
    //         format!(
    //             "Hosted by {} ({}) [{:.2} km]: {:.3} ms",
    //             best_server.sponsor, best_server.name, best_server.d, best_server.latency
    //         )
    //         .green()
    //     );
    // }

    // let mut results = speedtest.get_results()
    //     .ok_or_else(|| SpeedtestError::Unknown("Failed to get results".to_string()))?;
    
    // results.ping = best_server.latency;
    // results.server = best_server;

    // Clone best_server BEFORE calling get_results() to avoid borrow conflict
    let best_server = speedtest.get_best_server(None)?.clone();  // CLONE HERE
    if !quiet {
        println!(
            "{}",
            format!(
                "Hosted by {} ({}) [{:.2} km]: {:.3} ms",
                best_server.sponsor, best_server.name, best_server.d, best_server.latency
            )
            .green()
        );
    }
    let mut results = speedtest.get_results()
        .ok_or_else(|| SpeedtestError::Unknown("Failed to get results".to_string()))?;
    results.ping = best_server.latency;
    results.server = best_server;  // Store cloned server in results

    // Perform download test
    // if !args.no_download {
    //     if !quiet {
    //         print!("{}", "Testing download speed".cyan());
    //         if args.debug {
    //             println!();
    //         }
    //         std::io::Write::flush(&mut std::io::stdout()).unwrap();
    //     }

    //     let threads = if args.single { Some(1) } else { None };
        
    //     let pb = if !quiet && !args.debug {
    //         let pb = ProgressBar::new_spinner();
    //         pb.set_style(
    //             ProgressStyle::default_spinner()
    //                 .template("{spinner:.green} {msg}")
    //                 .unwrap(),
    //         );
    //         Some(pb)
    //     } else {
    //         None
    //     };

    //     let callback = move |_i: usize, _total: usize, _start: bool, _end: bool| {
    //         if let Some(ref pb) = pb {
    //             pb.tick();
    //         }
    //     };

    //     let download_speed = speedtest.download(callback, threads)?;
    //     results.download = download_speed;

    //     if let Some(pb) = pb {
    //         pb.finish_and_clear();
    //     }

    //     let divisor = if args.bytes { 8.0 } else { 1.0 };
    //     let unit = if args.bytes { "byte" } else { "bit" };

    //     if !quiet {
    //         println!(
    //             "{} {:.2} M{}/s",
    //             "Download:".green().bold(),
    //             (download_speed / 1_000_000.0) / divisor,
    //             unit
    //         );
    //     }
    // } else if !quiet {
    //     println!("{}", "Skipping download test".yellow());
    // }

    // Perform download test
    if !args.no_download {
        if !quiet {
            print!("{}", "Testing download speed".cyan());
            if args.debug {
                println!();
            }
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        let threads = if args.single { Some(1) } else { None };
        let pb = if !quiet && !args.debug {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            Some(pb)
        } else {
            None
        };
        
        // FIX: Clone pb BEFORE moving into closure
        let pb_for_callback = pb.clone();
        let callback = move |_i: usize, _total: usize, _start: bool, _end: bool| {
            if let Some(pb) = &pb_for_callback {
                pb.tick();
            }
        };
        
        let download_speed = speedtest.download(callback, threads)?;
        results.download = download_speed;
        
        // Now pb is still usable here
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        
        let divisor = if args.bytes { 8.0 } else { 1.0 };
        let unit = if args.bytes { "byte" } else { "bit" };
        if !quiet {
            println!(
                "{} {:.2} M{}/s",
                "Download:".green().bold(),
                (download_speed / 1_000_000.0) / divisor,
                unit
            );
        }
    } else if !quiet {
        println!("{}", "Skipping download test".yellow());
    }

    // Perform upload test
    // if !args.no_upload {
    //     if !quiet {
    //         print!("{}", "Testing upload speed".cyan());
    //         if args.debug {
    //             println!();
    //         }
    //         std::io::Write::flush(&mut std::io::stdout()).unwrap();
    //     }

    //     let threads = if args.single { Some(1) } else { None };
    //     let pre_allocate = !args.no_pre_allocate;

    //     let pb = if !quiet && !args.debug {
    //         let pb = ProgressBar::new_spinner();
    //         pb.set_style(
    //             ProgressStyle::default_spinner()
    //                 .template("{spinner:.green} {msg}")
    //                 .unwrap(),
    //         );
    //         Some(pb)
    //     } else {
    //         None
    //     };

    //     let callback = move |_i: usize, _total: usize, _start: bool, _end: bool| {
    //         if let Some(ref pb) = pb {
    //             pb.tick();
    //         }
    //     };

    //     let upload_speed = speedtest.upload(callback, threads, pre_allocate)?;
    //     results.upload = upload_speed;

    //     if let Some(pb) = pb {
    //         pb.finish_and_clear();
    //     }

    //     let divisor = if args.bytes { 8.0 } else { 1.0 };
    //     let unit = if args.bytes { "byte" } else { "bit" };

    //     if !quiet {
    //         println!(
    //             "{} {:.2} M{}/s",
    //             "Upload:".green().bold(),
    //             (upload_speed / 1_000_000.0) / divisor,
    //             unit
    //         );
    //     }
    // } else if !quiet {
    //     println!("{}", "Skipping upload test".yellow());
    // }

    // Perform upload test
    if !args.no_upload {
        if !quiet {
            print!("{}", "Testing upload speed".cyan());
            if args.debug {
                println!();
            }
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        let threads = if args.single { Some(1) } else { None };
        let pre_allocate = !args.no_pre_allocate;
        let pb = if !quiet && !args.debug {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            Some(pb)
        } else {
            None
        };
        
        // FIX: Clone pb BEFORE moving into closure
        let pb_for_callback = pb.clone();
        let callback = move |_i: usize, _total: usize, _start: bool, _end: bool| {
            if let Some(pb) = &pb_for_callback {
                pb.tick();
            }
        };
        
        let upload_speed = speedtest.upload(callback, threads, pre_allocate)?;
        results.upload = upload_speed;
        
        // Now pb is still usable here
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        
        let divisor = if args.bytes { 8.0 } else { 1.0 };
        let unit = if args.bytes { "byte" } else { "bit" };
        if !quiet {
            println!(
                "{} {:.2} M{}/s",
                "Upload:".green().bold(),
                (upload_speed / 1_000_000.0) / divisor,
                unit
            );
        }
    } else if !quiet {
        println!("{}", "Skipping upload test".yellow());
    }

    // Output results
    if args.simple {
        let divisor = if args.bytes { 8.0 } else { 1.0 };
        let unit = if args.bytes { "byte" } else { "bit" };
        
        println!("Ping: {:.3} ms", results.ping);
        println!(
            "Download: {:.2} M{}/s",
            (results.download / 1_000_000.0) / divisor,
            unit
        );
        println!(
            "Upload: {:.2} M{}/s",
            (results.upload / 1_000_000.0) / divisor,
            unit
        );
    } else if args.csv {
        let delimiter = args.csv_delimiter.chars().next().unwrap();
        println!("{}", results.to_csv(delimiter)?);
    } else if args.json {
        println!("{}", results.to_json(true)?);
    }

    if args.share && !args.csv && !args.json {
        println!("{}", "Share functionality not yet implemented".yellow());
    }

    Ok(())
}
