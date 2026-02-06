use crate::error::{Result, SpeedtestError};
use crate::speedtest::Speedtest;
use std::collections::HashMap;

impl Speedtest {
    /// Share results to speedtest.net and get share URL
    pub fn share_results(&mut self) -> Result<String> {
        if let Some(ref share_url) = self.results.share {
            let url: String = share_url.clone();
            return Ok(url);
        }

        let download = (self.results.download / 1000.0).round() as u64;
        let upload = (self.results.upload / 1000.0).round() as u64;
        let ping = self.results.ping.round() as u64;
        let server_id = self.results.server.id;

        // Calculate hash
        let hash_input = format!("{}-{}-{}-297aae72", ping, upload, download);
        let hash = format!("{:x}", md5::compute(hash_input.as_bytes()));

        // Build POST data
        let api_data = vec![
            format!("recommendedserverid={}", server_id),
            format!("ping={}", ping),
            "screenresolution=".to_string(),
            "promo=".to_string(),
            format!("download={}", download),
            "screendpi=".to_string(),
            format!("upload={}", upload),
            "testmethod=http".to_string(),
            format!("hash={}", hash),
            "touchscreen=none".to_string(),
            "startmode=pingselect".to_string(),
            "accuracy=1".to_string(),
            format!("bytesreceived={}", self.results.bytes_received),
            format!("bytessent={}", self.results.bytes_sent),
            format!("serverid={}", server_id),
        ];

        let post_data = api_data.join("&");

        let response = self
            .http_client
            .post(
                "://www.speedtest.net/api/api.php",
                post_data.into_bytes(),
            )
            .map_err(|e: reqwest::Error| SpeedtestError::ShareResultsConnectFailure(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SpeedtestError::ShareResultsSubmitFailure(
                "Could not submit results".to_string(),
            ));
        }

        let response_text = response
            .text()
            .map_err(|e: reqwest::Error| SpeedtestError::ShareResultsSubmitFailure(e.to_string()))?;

        // Parse result ID from response
        let params: HashMap<String, String> = url::form_urlencoded::parse(response_text.as_bytes())
            .into_owned()
            .collect();

        let result_id = params
            .get("resultid")
            .ok_or(SpeedtestError::ShareResultsSubmitFailure(
                "No result ID in response".to_string(),
            ))?;

        let share_url = format!("http://www.speedtest.net/result/{}.png", result_id);
        self.results.share = Some(share_url.clone());

        Ok(share_url)
    }
}