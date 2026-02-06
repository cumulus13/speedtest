use crate::error::{Result, SpeedtestError};
use crate::http::HttpClient;
use crate::types::*;
use crate::utils;
use std::collections::HashMap;

pub struct Speedtest {
    pub(crate) config: Option<Config>,
    pub(crate) http_client: HttpClient,
    pub(crate) servers: HashMap<String, Vec<Server>>,
    pub(crate) closest: Vec<Server>,
    pub(crate) best: Option<Server>,
    pub(crate) results: SpeedtestResults,
    pub(crate) lat_lon: Option<(f64, f64)>,
}

impl Speedtest {
    pub fn new(timeout: u64, source_address: Option<String>, secure: bool) -> Result<Self> {
        let http_client = HttpClient::new(timeout, source_address, secure)?;

        Ok(Self {
            config: None,
            http_client,
            servers: HashMap::new(),
            closest: Vec::new(),
            best: None,
            results: SpeedtestResults::default(),
            lat_lon: None,
        })
    }

    /// Download and parse speedtest configuration
    pub fn get_config(&mut self) -> Result<&Config> {
        let xml = self
            .http_client
            .get_text("://www.speedtest.net/speedtest-config.php")?;

        let config: SpeedtestConfig = quick_xml::de::from_str(&xml)
            .map_err(|e| SpeedtestError::ConfigError(format!("Failed to parse config: {}", e)))?;

        // Parse ignore servers
        let ignore_servers: Vec<u32> = config
            .server_config
            .ignore_ids
            .split(',')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect();

        // Parse upload ratio and sizes
        let ratio: usize = config
            .upload
            .ratio
            .parse()
            .map_err(|_| SpeedtestError::ConfigError("Invalid ratio".to_string()))?;

        let upload_max: usize = config
            .upload
            .max_chunk_count
            .parse()
            .map_err(|_| SpeedtestError::ConfigError("Invalid max chunk count".to_string()))?;

        let up_sizes = vec![32768, 65536, 131072, 262144, 524288, 1048576, 7340032];
        let upload_sizes = up_sizes[(ratio - 1)..].to_vec();
        let download_sizes = vec![350, 500, 750, 1000, 1500, 2000, 2500, 3000, 3500, 4000];

        let size_count = upload_sizes.len();
        let upload_count = (upload_max as f64 / size_count as f64).ceil() as usize;

        let download_threads: usize = config
            .download
            .threads_per_url
            .parse()
            .map_err(|_| SpeedtestError::ConfigError("Invalid threads per url".to_string()))?;

        let server_thread_count: usize = config
            .server_config
            .thread_count
            .parse()
            .map_err(|_| SpeedtestError::ConfigError("Invalid thread count".to_string()))?;

        let upload_threads: usize = config
            .upload
            .threads
            .parse()
            .map_err(|_| SpeedtestError::ConfigError("Invalid upload threads".to_string()))?;

        let download_length: u64 = config
            .download
            .test_length
            .parse()
            .map_err(|_| SpeedtestError::ConfigError("Invalid download test length".to_string()))?;

        let upload_length: u64 = config
            .upload
            .test_length
            .parse()
            .map_err(|_| SpeedtestError::ConfigError("Invalid upload test length".to_string()))?;

        // Parse client lat/lon
        let lat: f64 = config
            .client
            .lat
            .parse()
            .map_err(|_| SpeedtestError::InvalidLocation {
                lat: Some(config.client.lat.clone()),
                lon: Some(config.client.lon.clone()),
            })?;

        let lon: f64 = config
            .client
            .lon
            .parse()
            .map_err(|_| SpeedtestError::InvalidLocation {
                lat: Some(config.client.lat.clone()),
                lon: Some(config.client.lon.clone()),
            })?;

        self.lat_lon = Some((lat, lon));

        let parsed_config = Config {
            client: config.client.clone(),
            ignore_servers,
            sizes: Sizes {
                upload: upload_sizes,
                download: download_sizes,
            },
            counts: Counts {
                upload: upload_count,
                download: download_threads,
            },
            threads: Threads {
                upload: upload_threads,
                download: server_thread_count * 2,
            },
            length: Length {
                upload: upload_length,
                download: download_length,
            },
            upload_max: upload_count * size_count,
        };

        self.results.client = config.client;
        self.config = Some(parsed_config);

        Ok(self.config.as_ref().unwrap())
    }

    /// Get list of speedtest servers
    pub fn get_servers(
        &mut self,
        server_ids: Option<Vec<u32>>,
        exclude: Option<Vec<u32>>,
    ) -> Result<&HashMap<String, Vec<Server>>> {
        let urls = vec![
            "://www.speedtest.net/speedtest-servers-static.php",
            "http://c.speedtest.net/speedtest-servers-static.php",
            "://www.speedtest.net/speedtest-servers.php",
            "http://c.speedtest.net/speedtest-servers.php",
        ];

        let config = self
            .config
            .as_ref()
            .ok_or(SpeedtestError::ConfigError("Config not loaded".to_string()))?;

        let lat_lon = self
            .lat_lon
            .ok_or(SpeedtestError::ConfigError("Location not set".to_string()))?;

        self.servers.clear();

        for url in urls {
            match self.http_client.get_text(url) {
                Ok(xml) => {
                    if let Ok(root) = quick_xml::de::from_str::<ServersRoot>(&xml) {
                        for server_xml in root.servers.server {
                            let id: u32 = match server_xml.id.parse() {
                                Ok(id) => id,
                                Err(_) => continue,
                            };

                            // Filter by server IDs if specified
                            if let Some(ref ids) = server_ids {
                                if !ids.contains(&id) {
                                    continue;
                                }
                            }

                            // Exclude servers
                            if config.ignore_servers.contains(&id) {
                                continue;
                            }
                            if let Some(ref excl) = exclude {
                                if excl.contains(&id) {
                                    continue;
                                }
                            }

                            // Calculate distance
                            let server_lat: f64 = match server_xml.lat.parse() {
                                Ok(v) => v,
                                Err(_) => continue,
                            };
                            let server_lon: f64 = match server_xml.lon.parse() {
                                Ok(v) => v,
                                Err(_) => continue,
                            };

                            let d = utils::distance(lat_lon, (server_lat, server_lon));

                            let server = Server {
                                id,
                                sponsor: server_xml.sponsor,
                                name: server_xml.name,
                                country: server_xml.country,
                                country_code: server_xml.country_code,
                                host: server_xml.host,
                                url: server_xml.url,
                                lat: server_xml.lat,
                                lon: server_xml.lon,
                                d,
                                latency: 0.0,
                            };

                            let key = format!("{:.2}", d);
                            self.servers.entry(key).or_insert_with(Vec::new).push(server);
                        }
                        break;
                    }
                }
                Err(_) => continue,
            }
        }

        if (server_ids.is_some() || exclude.is_some()) && self.servers.is_empty() {
            return Err(SpeedtestError::NoMatchedServers);
        }

        Ok(&self.servers)
    }

    /// Get closest servers by distance
    pub fn get_closest_servers(&mut self, limit: usize) -> Result<&Vec<Server>> {
        if self.servers.is_empty() {
            self.get_servers(None, None)?;
        }

        self.closest.clear();

        let mut keys: Vec<_> = self.servers.keys().collect();
        keys.sort_by(|a, b| {
            let a_val: f64 = a.parse().unwrap_or(f64::MAX);
            let b_val: f64 = b.parse().unwrap_or(f64::MAX);
            a_val.partial_cmp(&b_val).unwrap()
        });

        for key in keys {
            if let Some(servers) = self.servers.get(key) {
                for server in servers {
                    self.closest.push(server.clone());
                    if self.closest.len() >= limit {
                        return Ok(&self.closest);
                    }
                }
            }
        }

        Ok(&self.closest)
    }

    pub fn get_config_ref(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    pub fn get_results(&self) -> &SpeedtestResults {
        &self.results
    }

    pub fn get_best_server(&self) -> Option<&Server> {
        self.best.as_ref()
    }
}