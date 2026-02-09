# Quick Start Guide - speedtest

Get running in 5 minutes!

## 1. Installation

### Linux
```bash
wget https://github.com/cumulus13/speedtest/releases/latest/download/speedtest-linux-x86_64
chmod +x speedtest-linux-x86_64
sudo mv speedtest-linux-x86_64 /usr/local/bin/speedtest
```

### macOS (Intel)
```bash
curl -L https://github.com/cumulus13/speedtest/releases/latest/download/speedtest-macos-x86_64 -o speedtest
chmod +x speedtest
sudo mv speedtest /usr/local/bin/
```

### macOS (Apple Silicon)
```bash
curl -L https://github.com/cumulus13/speedtest/releases/latest/download/speedtest-macos-aarch64 -o speedtest
chmod +x speedtest
sudo mv speedtest /usr/local/bin/
```

### Windows
```powershell
Invoke-WebRequest -Uri "https://github.com/cumulus13/speedtest/releases/latest/download/speedtest-windows-x86_64.exe" -OutFile "speedtest.exe"
```

### From Source
```bash
git clone https://github.com/cumulus13/speedtest.git
cd speedtest
cargo build --release
sudo cp target/release/speedtest /usr/local/bin/
```

## 2. Basic Usage

### Simple Test
```bash
speedtest --simple
```

Output:
```
Ping: 15.234 ms
Download: 95.23 Mbit/s
Upload: 45.67 Mbit/s
```

### JSON Output
```bash
speedtest --json
```

### CSV Output
```bash
speedtest --csv
```

## 3. Common Commands

```bash
# List servers
speedtest --list

# Test with specific server
speedtest --server 12345

# Download only
speedtest --no-upload

# Upload only
speedtest --no-download

# Save to file
speedtest --json > results.json
```

## 4. Docker

```bash
# Build
docker build -t speedtest .

# Run
docker run --rm --network host speedtest --simple
```

## 5. Troubleshooting

### Timeout issues
```bash
speedtest --timeout 30
```

### SSL errors
```bash
speedtest  # Try without --secure first
```

## Quick Reference

| Task | Command |
|------|---------|
| Basic test | `speedtest` |
| Simple output | `speedtest --simple` |
| JSON | `speedtest --json` |
| CSV | `speedtest --csv` |
| List servers | `speedtest --list` |
| Specific server | `speedtest --server ID` |
| Download only | `speedtest --no-upload` |
| Upload only | `speedtest --no-download` |

**Need Help?** 
- GitHub: https://github.com/cumulus13/speedtest/issues
- Email: cumulus13@gmail.com
