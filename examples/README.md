# speedtest Examples

This directory contains examples of how to use speedtest in various scenarios.

## Basic Usage Examples

### 1. Simple Speed Test
```bash
speedtest --simple
```

Output:
```
Ping: 15.234 ms
Download: 95.23 Mbit/s
Upload: 45.67 Mbit/s
```

### 2. JSON Output for Scripting
```bash
speedtest --json > results.json
```

### 3. CSV Output for Data Analysis
```bash
speedtest --csv >> speedtest-log.csv
```

### 4. Testing with Specific Server
```bash
# First, list available servers
speedtest --list | head -20

# Then test with a specific server
speedtest --server 12345
```

### 5. Download Only Test
```bash
speedtest --no-upload --simple
```

### 6. Upload Only Test
```bash
speedtest --no-download --simple
```

### 7. Single Connection Test
Simulates a typical file transfer:
```bash
speedtest --single --simple
```

### 8. Secure HTTPS Connection
```bash
speedtest --secure --simple
```

### 9. Custom Timeout
For slower or unreliable connections:
```bash
speedtest --timeout 30 --simple
```

### 10. Multiple Server Exclusion
```bash
speedtest --exclude 12345 --exclude 67890
```

## Advanced Examples

### Monitoring Script (Bash)

Create `monitor-speed.sh`:
```bash
#!/bin/bash

# Run speed test every hour and log results
while true; do
    echo "$(date): Running speed test..."
    speedtest --csv >> /var/log/speedtest-$(date +%Y-%m).csv
    sleep 3600
done
```

### Python Integration

```python
import subprocess
import json

def run_speedtest():
    result = subprocess.run(
        ['speedtest', '--json'],
        capture_output=True,
        text=True
    )
    return json.loads(result.stdout)

data = run_speedtest()
print(f"Download: {data['download'] / 1_000_000:.2f} Mbit/s")
print(f"Upload: {data['upload'] / 1_000_000:.2f} Mbit/s")
print(f"Ping: {data['ping']:.3f} ms")
```

### PowerShell Integration (Windows)

```powershell
# Run speedtest and parse JSON
$result = speedtest --json | ConvertFrom-Json

Write-Host "Download: $($result.download / 1MB) Mbit/s"
Write-Host "Upload: $($result.upload / 1MB) Mbit/s"
Write-Host "Ping: $($result.ping) ms"

# Log to file
$result | ConvertTo-Json | Out-File -Append speedtest-log.json
```

### Cron Job Setup (Linux)

Add to crontab (`crontab -e`):
```
# Run speedtest every 4 hours and log results
0 */4 * * * /usr/local/bin/speedtest --csv >> /var/log/speedtest.csv 2>&1

# Daily summary at midnight
0 0 * * * /usr/local/bin/speedtest --json > /var/log/speedtest-$(date +\%Y-\%m-\%d).json
```

### Docker Usage

```bash
# Build image
docker build -t speedtest .

# Run simple test
docker run --rm --network host speedtest --simple

# Run with JSON output
docker run --rm --network host speedtest --json

# Save results to file
docker run --rm --network host speedtest --json > results.json
```

### GitHub Actions Integration

Create `.github/workflows/speedtest.yml`:
```yaml
name: Network Speed Test

on:
  schedule:
    - cron: '0 0 * * *'  # Daily at midnight
  workflow_dispatch:

jobs:
  speedtest:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Download speedtest binary
        run: |
          wget https://github.com/cumulus13/speedtest/releases/latest/download/speedtest-linux-x86_64
          chmod +x speedtest-linux-x86_64
          
      - name: Run speed test
        run: ./speedtest-linux-x86_64 --json > speedtest-result.json
        
      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: speedtest-results
          path: speedtest-result.json
```

### Prometheus Integration

Create a script to export metrics:
```bash
#!/bin/bash

# Run speedtest and extract metrics
RESULT=$(speedtest --json)

DOWNLOAD=$(echo $RESULT | jq -r '.download')
UPLOAD=$(echo $RESULT | jq -r '.upload')
PING=$(echo $RESULT | jq -r '.ping')

# Export to Prometheus textfile collector
cat > /var/lib/node_exporter/speedtest.prom <<EOF
# HELP speedtest_download_bits Download speed in bits per second
# TYPE speedtest_download_bits gauge
speedtest_download_bits $DOWNLOAD

# HELP speedtest_upload_bits Upload speed in bits per second
# TYPE speedtest_upload_bits gauge
speedtest_upload_bits $UPLOAD

# HELP speedtest_ping_ms Ping latency in milliseconds
# TYPE speedtest_ping_ms gauge
speedtest_ping_ms $PING
EOF
```

### Grafana Dashboard Query

```promql
# Download speed in Mbit/s
speedtest_download_bits / 1000000

# Upload speed in Mbit/s
speedtest_upload_bits / 1000000

# Ping latency
speedtest_ping_ms
```

## Troubleshooting Examples

### Debug Mode
```bash
# Enable verbose output
RUST_LOG=debug speedtest --simple
```

### Network Issues
```bash
# Test with increased timeout
speedtest --timeout 60 --simple

# Try without HTTPS
speedtest --simple

# Try with HTTPS
speedtest --secure --simple
```

### Performance Testing
```bash
# Test with single thread
speedtest --single --simple

# Test with default multi-threading
speedtest --simple

# Compare results
diff <(speedtest --single --json) <(speedtest --json)
```

## Integration Examples

### Ansible Playbook

```yaml
---
- name: Network Speed Test
  hosts: all
  tasks:
    - name: Install speedtest
      get_url:
        url: https://github.com/cumulus13/speedtest/releases/latest/download/speedtest-linux-x86_64
        dest: /usr/local/bin/speedtest
        mode: '0755'
    
    - name: Run speed test
      command: speedtest --json
      register: speedtest_result
    
    - name: Display results
      debug:
        var: speedtest_result.stdout
```

### Terraform

```hcl
resource "null_resource" "speedtest" {
  provisioner "local-exec" {
    command = "speedtest --json > ${path.module}/speedtest-result.json"
  }
}

data "local_file" "speedtest_results" {
  depends_on = [null_resource.speedtest]
  filename   = "${path.module}/speedtest-result.json"
}

output "network_speed" {
  value = jsondecode(data.local_file.speedtest_results.content)
}
```

## API Usage (Library)

If using speedtest as a library in your Rust project:

```rust
use speedtest::{Speedtest, Result};

fn main() -> Result<()> {
    // Initialize
    let mut speedtest = Speedtest::new(10, false, None)?;
    
    // Get configuration
    speedtest.get_config()?;
    
    // Get best server
    speedtest.get_best_server(None)?;
    
    // Run download test
    let download_speed = speedtest.download(
        |_i, _total, _start, _end| {},
        None
    )?;
    
    // Run upload test
    let upload_speed = speedtest.upload(
        |_i, _total, _start, _end| {},
        None,
        true
    )?;
    
    println!("Download: {:.2} Mbit/s", download_speed / 1_000_000.0);
    println!("Upload: {:.2} Mbit/s", upload_speed / 1_000_000.0);
    
    Ok(())
}
```

Add to `Cargo.toml`:
```toml
[dependencies]
speedtest = "1.0"
```
