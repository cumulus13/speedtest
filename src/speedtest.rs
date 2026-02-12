// File: src\speedtest.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-09
// Description: Command-line interface for testing internet bandwidth using speedtest.net
// License: MIT

use crate::error::{Result, SpeedtestError};
use crate::http::HttpClient;
use crate::models::*;
use crate::utils::distance;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct Speedtest {
    config: Option<Config>,
    client: HttpClient,
    servers: HashMap<u32, Vec<Server>>,
    closest: Vec<Server>,
    best: Option<Server>,
    lat_lon: (f64, f64),
    debug: bool,
}

impl Speedtest {
    pub fn new(timeout: u64, secure: bool, source_address: Option<String>) -> Result<Self> {
        let client = HttpClient::new(timeout, secure, source_address)?;

        Ok(Self {
            config: None,
            client,
            servers: HashMap::new(),
            closest: Vec::new(),
            best: None,
            lat_lon: (0.0, 0.0),
            debug: false,
        })
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn get_config(&mut self) -> Result<&Config> {
        if self.config.is_some() {
            return Ok(self.config.as_ref().unwrap());
        }

        let xml = self
            .client
            .get_text("://www.speedtest.net/speedtest-config.php")?;

        // Parse XML manually to extract attributes
        use quick_xml::events::Event;
        use quick_xml::Reader;
        use std::collections::HashMap;

        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);

        let mut client_attrs: HashMap<String, String> = HashMap::new();
        let mut server_config_attrs: HashMap<String, String> = HashMap::new();
        let mut download_attrs: HashMap<String, String> = HashMap::new();
        let mut upload_attrs: HashMap<String, String> = HashMap::new();

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    
                    let attrs: HashMap<String, String> = e
                        .attributes()
                        .filter_map(|a| a.ok())
                        .map(|a| {
                            (
                                String::from_utf8_lossy(a.key.as_ref()).to_string(),
                                String::from_utf8_lossy(&a.value).to_string(),
                            )
                        })
                        .collect();

                    match name.as_str() {
                        "client" => client_attrs = attrs,
                        "server-config" => server_config_attrs = attrs,
                        "download" => download_attrs = attrs,
                        "upload" => upload_attrs = attrs,
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(SpeedtestError::ConfigRetrieval(format!(
                        "XML parse error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    )))
                }
                _ => {}
            }
            buf.clear();
        }

        // Build Client from attributes
        let client = Client {
            ip: client_attrs.get("ip").cloned().unwrap_or_default(),
            lat: client_attrs.get("lat").cloned().unwrap_or_default(),
            lon: client_attrs.get("lon").cloned().unwrap_or_default(),
            isp: client_attrs.get("isp").cloned().unwrap_or_default(),
            country: client_attrs.get("country").cloned().unwrap_or_default(),
            isprating: client_attrs.get("isprating").cloned().unwrap_or_default(),
            rating: client_attrs.get("rating").cloned().unwrap_or_default(),
            ispdlavg: client_attrs.get("ispdlavg").cloned().unwrap_or_default(),
            ispulavg: client_attrs.get("ispulavg").cloned().unwrap_or_default(),
            loggedin: client_attrs.get("loggedin").cloned().unwrap_or_default(),
        };

        // Validate client data
        if client.ip.is_empty() {
            return Err(SpeedtestError::ConfigRetrieval(
                "Client IP address not provided by server".to_string()
            ));
        }

        let ignore_servers: Vec<u32> = server_config_attrs
            .get("ignoreids")
            .unwrap_or(&String::new())
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let ratio: usize = upload_attrs
            .get("ratio")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        
        let upload_max: usize = upload_attrs
            .get("maxchunkcount")
            .and_then(|s| s.parse().ok())
            .unwrap_or(4);

        let up_sizes = vec![32768, 65536, 131072, 262144, 524288, 1048576, 7340032];
        let upload_sizes: Vec<usize> = if ratio > 0 && ratio <= up_sizes.len() {
            up_sizes[(ratio - 1)..].to_vec()
        } else {
            up_sizes
        };

        let size_count = upload_sizes.len();
        let upload_count = (upload_max as f64 / size_count as f64).ceil() as usize;

        let lat: f64 = client.lat.parse()
            .unwrap_or_else(|_| {
                eprintln!("Warning: Could not parse latitude '{}', using default 0.0", client.lat);
                0.0
            });
        let lon: f64 = client.lon.parse()
            .unwrap_or_else(|_| {
                eprintln!("Warning: Could not parse longitude '{}', using default 0.0", client.lon);
                0.0
            });

        self.lat_lon = (lat, lon);

        let config = Config {
            client,
            ignore_servers,
            sizes: Sizes {
                upload: upload_sizes,
                download: vec![350, 500, 750, 1000, 1500, 2000, 2500, 3000, 3500, 4000],
            },
            counts: Counts {
                upload: upload_count,
                download: download_attrs
                    .get("threadsperurl")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(4),
            },
            threads: Threads {
                upload: upload_attrs
                    .get("threads")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(8),
                download: server_config_attrs
                    .get("threadcount")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(4)
                    * 2,
            },
            length: Length {
                upload: upload_attrs
                    .get("testlength")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
                download: download_attrs
                    .get("testlength")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
            },
            upload_max: upload_count * size_count,
        };

        self.config = Some(config);
        Ok(self.config.as_ref().unwrap())
    }

    pub fn get_servers(
        &mut self,
        server_ids: Option<&[u32]>,
        exclude: Option<&[u32]>,
    ) -> Result<&HashMap<u32, Vec<Server>>> {
        self.servers.clear();

        let urls = vec![
            "://www.speedtest.net/speedtest-servers-static.php",
            "http://c.speedtest.net/speedtest-servers-static.php",
            "://www.speedtest.net/speedtest-servers.php",
            "http://c.speedtest.net/speedtest-servers.php",
        ];

        for url in urls {
            if self.debug {
                eprintln!("Trying to fetch servers from: {}", url);
            }
            match self.fetch_servers(url, server_ids, exclude) {
                Ok(_) if !self.servers.is_empty() => {
                    if self.debug {
                        eprintln!("Found {} unique servers", self.servers.len());
                    }
                    break;
                }
                Ok(_) => {
                    if self.debug {
                        eprintln!("No servers found from this URL");
                    }
                }
                Err(e) => {
                    if self.debug {
                        eprintln!("Failed to fetch: {}", e);
                    }
                    continue;
                }
            }
        }

        if (server_ids.is_some() || exclude.is_some()) && self.servers.is_empty() {
            return Err(SpeedtestError::NoMatchedServers);
        }

        if self.debug {
            eprintln!("Total servers available: {}", self.servers.len());
        }
        Ok(&self.servers)
    }

    fn fetch_servers(
        &mut self,
        url: &str,
        server_ids: Option<&[u32]>,
        exclude: Option<&[u32]>,
    ) -> Result<()> {
        let xml = self.client.get_text(url)?;

        // Parse XML manually to extract server attributes
        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);

        let config = self.config.as_ref()
            .ok_or_else(|| SpeedtestError::ConfigRetrieval("Config not loaded".to_string()))?;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    
                    if name == "server" {
                        let attrs: HashMap<String, String> = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .map(|a| {
                                (
                                    String::from_utf8_lossy(a.key.as_ref()).to_string(),
                                    String::from_utf8_lossy(&a.value).to_string(),
                                )
                            })
                            .collect();

                        let id: u32 = attrs.get("id")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);

                        if id == 0 {
                            continue;
                        }

                        if let Some(ids) = server_ids {
                            if !ids.contains(&id) {
                                continue;
                            }
                        }

                        if config.ignore_servers.contains(&id) {
                            continue;
                        }

                        if let Some(excl) = exclude {
                            if excl.contains(&id) {
                                continue;
                            }
                        }

                        let lat: f64 = attrs.get("lat")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);
                        let lon: f64 = attrs.get("lon")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);

                        let d = distance(self.lat_lon.0, self.lat_lon.1, lat, lon);

                        let server = Server {
                            id,
                            sponsor: attrs.get("sponsor").cloned().unwrap_or_default(),
                            name: attrs.get("name").cloned().unwrap_or_default(),
                            country: attrs.get("country").cloned().unwrap_or_default(),
                            lat,
                            lon,
                            url: attrs.get("url").cloned().unwrap_or_default(),
                            d,
                            latency: 0.0,
                        };

                        self.servers.entry(id).or_insert_with(Vec::new).push(server);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(SpeedtestError::ServersRetrieval(format!(
                        "XML parse error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    )))
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }

    pub fn get_closest_servers(&mut self, limit: usize) -> Result<&[Server]> {
        if self.servers.is_empty() {
            self.get_servers(None, None)?;
        }

        let mut all_servers: Vec<Server> = self
            .servers
            .values()
            .flatten()
            .cloned()
            .collect();

        if self.debug {
            eprintln!("Total servers before sorting: {}", all_servers.len());
        }

        all_servers.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

        self.closest = all_servers.into_iter().take(limit).collect();

        if self.debug {
            eprintln!("Closest {} servers:", self.closest.len());
            for (i, s) in self.closest.iter().enumerate() {
                eprintln!("  {}. {} ({}) - {:.2} km", i+1, s.sponsor, s.name, s.d);
            }
        }

        Ok(&self.closest)
    }

    pub fn get_best_server(&mut self, servers: Option<&[Server]>) -> Result<&Server> {
        let servers_to_test = if let Some(s) = servers {
            s.to_vec()
        } else {
            if self.closest.is_empty() {
                self.get_closest_servers(5)?;
            }
            self.closest.clone()
        };

        let results: Vec<(f64, Server)> = servers_to_test
            .par_iter()
            .filter_map(|server| {
                let latency = self.measure_latency(server).ok()?;
                Some((latency, server.clone()))
            })
            .collect();

        let best = results
            .into_iter()
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .ok_or_else(|| SpeedtestError::BestServerFailure(
                "Unable to connect to servers to test latency".to_string()
            ))?;

        let mut best_server = best.1;
        best_server.latency = best.0;
        self.best = Some(best_server);

        Ok(self.best.as_ref().unwrap())
    }

    fn measure_latency(&self, server: &Server) -> Result<f64> {
        let url_parts: Vec<&str> = server.url.split('/').collect();
        let base_url = url_parts[..url_parts.len() - 1].join("/");

        if self.debug {
            eprintln!("Testing latency for server: {} ({})", server.sponsor, server.name);
            eprintln!("  Server URL: {}", server.url);
            eprintln!("  Base URL: {}", base_url);
        }

        let mut latencies = Vec::new();

        for i in 0..3 {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            
            let url = format!("{}/latency.txt?x={}.{}", base_url, timestamp, i);
            
            if self.debug {
                eprintln!("  Attempt {} - Testing URL: {}", i+1, url);
            }
            
            let start = Instant::now();
            match self.client.get_text(&url) {
                Ok(response) if response.trim() == "test=test" => {
                    let latency = start.elapsed().as_secs_f64() * 1000.0;
                    if self.debug {
                        eprintln!("  SUCCESS - Latency: {:.3} ms", latency);
                    }
                    latencies.push(latency);
                }
                Ok(response) => {
                    if self.debug {
                        eprintln!("  Unexpected response: '{}'", response.trim());
                    }
                    latencies.push(3600.0);
                }
                Err(e) => {
                    if self.debug {
                        eprintln!("  Error: {}", e);
                    }
                    latencies.push(3600.0);
                }
            }
        }

        if latencies.iter().all(|&l| l >= 3600.0) {
            return Err(SpeedtestError::BestServerFailure(
                format!("All latency tests failed for {}", server.sponsor)
            ));
        }

        let avg = latencies.iter().sum::<f64>() / latencies.len() as f64;
        
        if self.debug {
            eprintln!("  Average latency: {:.3} ms", avg);
        }
        
        Ok((avg * 1000.0).round() / 1000.0)
    }

    pub fn download<F>(&self, _callback: F, threads: Option<usize>) -> Result<f64>
    where
        F: Fn(usize, usize, bool, bool) + Send + Sync,
    {
        let config = self.config.as_ref()
            .ok_or_else(|| SpeedtestError::ConfigRetrieval("Config not loaded".to_string()))?;
        let server = self.best.as_ref()
            .ok_or(SpeedtestError::MissingBestServer)?;

        let base_url = server.url.split('/').collect::<Vec<_>>();
        let base_url = base_url[..base_url.len() - 1].join("/");

        let mut urls = Vec::new();
        for size in &config.sizes.download {
            for _ in 0..config.counts.download {
                urls.push(format!("{}/random{}x{}.jpg", base_url, size, size));
            }
        }

        if self.debug {
            eprintln!("Download test configuration:");
            eprintln!("  Base URL: {}", base_url);
            eprintln!("  Total URLs: {}", urls.len());
            eprintln!("  Threads: {}", threads.unwrap_or(config.threads.download));
            eprintln!("  Test duration: {} seconds", config.length.download);
        }

        let max_threads = threads.unwrap_or(config.threads.download);
        let test_duration = Duration::from_secs(config.length.download);
        
        let total_bytes = Arc::new(AtomicU64::new(0));
        let start_time = Instant::now();
        let stop_flag = Arc::new(AtomicBool::new(false));

        // Spawn download threads
        let handles: Vec<_> = (0..max_threads)
            .map(|_| {
                let urls = urls.clone();
                let total_bytes = Arc::clone(&total_bytes);
                let stop_flag = Arc::clone(&stop_flag);
                let client = HttpClient::new(10, false, None).unwrap();
                
                std::thread::spawn(move || {
                    let mut url_index = 0;
                    while !stop_flag.load(Ordering::Relaxed) {
                        // Loop back to start when we reach the end
                        if url_index >= urls.len() {
                            url_index = 0;
                        }
                        
                        if let Ok(data) = client.get_bytes(&urls[url_index]) {
                            total_bytes.fetch_add(data.len() as u64, Ordering::Relaxed);
                        }
                        
                        url_index += 1;
                    }
                })
            })
            .collect();

        // Monitor progress
        std::thread::sleep(test_duration);
        stop_flag.store(true, Ordering::Relaxed);

        for handle in handles {
            let _ = handle.join();
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        let bytes = total_bytes.load(Ordering::Relaxed);
        let speed = (bytes as f64 / elapsed) * 8.0;

        if self.debug {
            eprintln!("Download test results:");
            eprintln!("  Bytes downloaded: {}", bytes);
            eprintln!("  Time elapsed: {:.2} seconds", elapsed);
            eprintln!("  Speed: {:.2} bits/s ({:.2} Mbit/s)", speed, speed / 1_000_000.0);
        }

        Ok(speed)
    }

    pub fn upload<F>(&self, _callback: F, threads: Option<usize>, _pre_allocate: bool) -> Result<f64>
    where
        F: Fn(usize, usize, bool, bool) + Send + Sync,
    {
        let config = self.config.as_ref()
            .ok_or_else(|| SpeedtestError::ConfigRetrieval("Config not loaded".to_string()))?;
        let server = self.best.as_ref()
            .ok_or(SpeedtestError::MissingBestServer)?;

        let mut sizes = Vec::new();
        for size in &config.sizes.upload {
            for _ in 0..config.counts.upload {
                sizes.push(*size);
            }
        }

        if self.debug {
            eprintln!("Upload test configuration:");
            eprintln!("  Server URL: {}", server.url);
            eprintln!("  Total data chunks: {}", sizes.len());
            eprintln!("  Threads: {}", threads.unwrap_or(config.threads.upload));
            eprintln!("  Test duration: {} seconds", config.length.upload);
        }

        let max_threads = threads.unwrap_or(config.threads.upload);
        let test_duration = Duration::from_secs(config.length.upload);
        
        let total_bytes = Arc::new(AtomicU64::new(0));
        let start_time = Instant::now();
        let stop_flag = Arc::new(AtomicBool::new(false));

        // Generate upload data
        let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let upload_data: Vec<Vec<u8>> = sizes
            .iter()
            .map(|&size| {
                let multiplier = (size as f64 / 36.0).round() as usize;
                let content = chars.repeat(multiplier);
                let data = format!("content1={}", &content[..size.min(content.len()) - 9]);
                data.into_bytes()
            })
            .collect();

        let handles: Vec<_> = (0..max_threads)
            .map(|i| {
                let data_chunk = upload_data.clone();
                let url = server.url.clone();
                let total_bytes = Arc::clone(&total_bytes);
                let stop_flag = Arc::clone(&stop_flag);
                let client = HttpClient::new(10, false, None).unwrap();
                
                std::thread::spawn(move || {
                    let mut data_index = i;
                    while !stop_flag.load(Ordering::Relaxed) {
                        // Loop back to start when we reach the end
                        if data_index >= data_chunk.len() {
                            data_index = i; // Start from this thread's offset again
                        }
                        
                        if let Ok(_) = client.post(&url, data_chunk[data_index].clone()) {
                            total_bytes.fetch_add(data_chunk[data_index].len() as u64, Ordering::Relaxed);
                        }
                        
                        data_index += max_threads;
                    }
                })
            })
            .collect();

        // Monitor progress
        std::thread::sleep(test_duration);
        stop_flag.store(true, Ordering::Relaxed);

        for handle in handles {
            let _ = handle.join();
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        let bytes = total_bytes.load(Ordering::Relaxed);
        let speed = (bytes as f64 / elapsed) * 8.0;

        if self.debug {
            eprintln!("Upload test results:");
            eprintln!("  Bytes uploaded: {}", bytes);
            eprintln!("  Time elapsed: {:.2} seconds", elapsed);
            eprintln!("  Speed: {:.2} bits/s ({:.2} Mbit/s)", speed, speed / 1_000_000.0);
        }

        Ok(speed)
    }

    pub fn get_results(&self) -> Option<SpeedtestResults> {
        let config = self.config.as_ref()?;
        let server = self.best.as_ref()?;

        Some(SpeedtestResults::new(
            config.client.clone(),
            server.clone(),
        ))
    }
}

use crate::utils::cache_buster;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speedtest_creation() {
        let st = Speedtest::new(10, false, None);
        assert!(st.is_ok());
    }
}
