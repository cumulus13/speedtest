use crate::error::Result;
use crate::http::HttpClient;
use crate::speedtest::Speedtest;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

impl Speedtest {
    /// Test upload speed
    pub fn test_upload<F>(&mut self, callback: Option<F>, pre_allocate: bool) -> Result<f64>
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

        let url = best.url.clone();

        // Build upload data sizes
        let mut sizes = Vec::new();
        for size in &config.sizes.upload {
            for _ in 0..config.counts.upload {
                sizes.push(*size);
            }
        }

        let request_count = config.upload_max.min(sizes.len());
        let sizes = sizes[..request_count].to_vec();

        let max_threads = config.threads.upload;
        let test_length = Duration::from_secs(config.length.upload);

        let bytes_sent = Arc::new(AtomicU64::new(0));
        let start = Instant::now();

        // Pre-allocate data if requested
        let data_chunks: Vec<Vec<u8>> = if pre_allocate {
            sizes
                .iter()
                .map(|size| self.generate_upload_data(*size))
                .collect()
        } else {
            Vec::new()
        };

        // Create a separate HTTP client for thread safety
        let client = HttpClient::new(10, None, false)?;
        let callback = Arc::new(callback);

        // Use rayon for parallel uploads
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(max_threads)
            .build()
            .unwrap();

        pool.install(|| {
            sizes.par_iter().enumerate().for_each(|(i, size)| {
                if start.elapsed() > test_length {
                    return;
                }

                let data = if pre_allocate {
                    data_chunks[i].clone()
                } else {
                    // Generate data on the fly
                    generate_upload_data_static(*size)
                };

                match client.post(&url, data.clone()) {
                    Ok(response) => {
                        if response.status().is_success() {
                            bytes_sent.fetch_add(data.len() as u64, Ordering::SeqCst);
                        }
                        if let Some(ref cb) = *callback {
                            cb(i + 1, request_count);
                        }
                    }
                    Err(_) => {}
                }
            });
        });

        let elapsed = start.elapsed().as_secs_f64();
        let total_bytes = bytes_sent.load(Ordering::SeqCst);

        self.results.bytes_sent = total_bytes;
        let speed = (total_bytes as f64 / elapsed) * 8.0; // Convert to bits per second
        self.results.upload = speed;

        Ok(speed)
    }

    /// Generate upload data of specified size
    fn generate_upload_data(&self, length: usize) -> Vec<u8> {
        generate_upload_data_static(length)
    }
}

/// Generate upload data (static version for use in closures)
fn generate_upload_data_static(length: usize) -> Vec<u8> {
    const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let multiplier = ((length as f64) / 36.0).round() as usize;

    let content_length = length.saturating_sub(9); // "content1=" is 9 chars
    let mut data = Vec::with_capacity(length);
    data.extend_from_slice(b"content1=");

    let pattern: Vec<u8> = CHARS.iter().cycle().take(multiplier).copied().collect();

    let mut pos = 0;
    while data.len() < length {
        let remaining = content_length - (data.len() - 9);
        let chunk_size = remaining.min(pattern.len());
        data.extend_from_slice(&pattern[..chunk_size]);
        pos += chunk_size;
    }

    // Ensure exact length
    data.truncate(length);

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_upload_data() {
        let data = generate_upload_data_static(100);
        assert_eq!(data.len(), 100);
        assert!(data.starts_with(b"content1="));
    }
}
