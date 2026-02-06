use crate::error::Result;
use crate::http::HttpClient;
use crate::speedtest::Speedtest;
use crate::utils;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

impl Speedtest {
    /// Test download speed
    pub fn test_download<F>(&mut self, callback: Option<F>) -> Result<f64>
    where
        F: Fn(usize, usize) + Send + Sync,
    {
        let config = self
            .config
            .as_ref()
            .ok_or(crate::error::SpeedtestError::ConfigError(
                "Config not loaded".to_string(),
            ))?;

        let best = self
            .best
            .as_ref()
            .ok_or(crate::error::SpeedtestError::MissingBestServer)?;

        let base_url = self.extract_base_url(&best.url);

        // Build URLs for download
        let mut urls = Vec::new();
        for size in &config.sizes.download {
            for _ in 0..config.counts.download {
                urls.push(format!("{}/random{}x{}.jpg", base_url, size, size));
            }
        }

        let total_requests = urls.len();
        let max_threads = config.threads.download;
        let test_length = Duration::from_secs(config.length.download);

        let bytes_received = Arc::new(AtomicU64::new(0));
        let start = Instant::now();

        // Create a separate HTTP client for thread safety
        let client = HttpClient::new(10, None, false)?;
        let callback = Arc::new(callback);

        // Use rayon for parallel downloads with thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(max_threads)
            .build()
            .unwrap();

        pool.install(|| {
            urls.par_iter().enumerate().for_each(|(i, url)| {
                if start.elapsed() > test_length {
                    return;
                }

                let url_with_cache_bust = utils::add_query_param(url, &utils::cache_bust());

                match client.get_bytes(&url_with_cache_bust) {
                    Ok(data) => {
                        bytes_received.fetch_add(data.len() as u64, Ordering::SeqCst);
                        if let Some(ref cb) = *callback {
                            cb(i + 1, total_requests);
                        }
                    }
                    Err(_) => {}
                }
            });
        });

        let elapsed = start.elapsed().as_secs_f64();
        let total_bytes = bytes_received.load(Ordering::SeqCst);

        self.results.bytes_received = total_bytes;
        let speed = (total_bytes as f64 / elapsed) * 8.0; // Convert to bits per second
        self.results.download = speed;

        // Adjust upload threads based on download speed
        if speed > 100_000.0 {
            if let Some(ref mut cfg) = self.config {
                cfg.threads.upload = 8;
            }
        }

        Ok(speed)
    }
}
