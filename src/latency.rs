use crate::error::{Result, SpeedtestError};
use crate::speedtest::Speedtest;
use crate::types::{ResultServer, Server};
use std::time::Instant;

impl Speedtest {
    /// Determine best server based on latency
    pub fn determine_best_server(&mut self, servers: Option<Vec<Server>>) -> Result<&Server> {
        let test_servers = if let Some(s) = servers {
            s
        } else {
            if self.closest.is_empty() {
                self.get_closest_servers(5)?;
            }
            self.closest.clone()
        };

        if test_servers.is_empty() {
            return Err(SpeedtestError::BestServerFailure);
        }

        let mut results = Vec::new();

        for server in &test_servers {
            let mut cumulative = Vec::new();
            let url = self.extract_base_url(&server.url);

            for i in 0..3 {
                let latency_url = format!(
                    "{}/latency.txt?x={}.{}",
                    url,
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    i
                );

                let start = Instant::now();
                match self.http_client.get_text(&latency_url) {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        if response.trim() == "test=test" {
                            cumulative.push(elapsed.as_secs_f64());
                        } else {
                            cumulative.push(3600.0);
                        }
                    }
                    Err(_) => {
                        cumulative.push(3600.0);
                    }
                }
            }

            let avg = (cumulative.iter().sum::<f64>() / 6.0) * 1000.0;
            results.push((avg, server.clone()));
        }

        results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        if results.is_empty() {
            return Err(SpeedtestError::BestServerFailure);
        }

        let (latency, mut best_server) = results.into_iter().next().unwrap();
        best_server.latency = latency;

        self.results.ping = latency;
        self.results.server = ResultServer {
            id: best_server.id,
            sponsor: best_server.sponsor.clone(),
            name: best_server.name.clone(),
            country: best_server.country.clone(),
            d: best_server.d,
            latency: best_server.latency,
            url: best_server.url.clone(),
        };

        self.best = Some(best_server);

        Ok(self.best.as_ref().unwrap())
    }

    fn extract_base_url(&self, url: &str) -> String {
        if let Some(pos) = url.rfind('/') {
            url[..pos].to_string()
        } else {
            url.to_string()
        }
    }
}
