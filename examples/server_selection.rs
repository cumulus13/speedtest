use speedtest::{Result, Speedtest};

fn main() -> Result<()> {
    println!("Speedtest Example - Server Selection\n");

    let mut speedtest = Speedtest::new(10, None, false)?;

    // Get configuration
    println!("Retrieving configuration...");
    speedtest.get_config()?;

    // Get all servers
    println!("Retrieving server list...\n");
    speedtest.get_servers(None, None)?;

    // Display all servers sorted by distance
    let mut all_servers = Vec::new();
    for servers in speedtest.get_servers(None, None)?.values() {
        all_servers.extend(servers.iter().cloned());
    }
    all_servers.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

    println!("Available servers (sorted by distance):\n");
    for (i, server) in all_servers.iter().take(10).enumerate() {
        println!(
            "{:2}. ID: {:5} | {} ({}, {}) [{:.2} km]",
            i + 1,
            server.id,
            server.sponsor,
            server.name,
            server.country,
            server.d
        );
    }

    // Get closest servers
    println!("\n\nGetting 5 closest servers...\n");
    let closest = speedtest.get_closest_servers(5)?;

    for (i, server) in closest.iter().enumerate() {
        println!(
            "{:2}. ID: {:5} | {} ({}) [{:.2} km]",
            i + 1,
            server.id,
            server.sponsor,
            server.name,
            server.d
        );
    }

    // Determine best server
    println!("\n\nDetermining best server based on latency...\n");
    speedtest.determine_best_server(None)?;

    let best = speedtest.get_best_server().unwrap();
    println!("Best Server:");
    println!("  ID: {}", best.id);
    println!("  Sponsor: {}", best.sponsor);
    println!("  Location: {}, {}", best.name, best.country);
    println!("  Distance: {:.2} km", best.d);
    println!("  Latency: {:.2} ms", best.latency);

    Ok(())
}
