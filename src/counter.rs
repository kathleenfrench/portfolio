// finder counts for the treasure hunt and the ctf, kept in upstash redis via
// its rest api. optional: without UPSTASH_REDIS_REST_URL/_TOKEN (e.g. no .env
// locally) there's no counter, and the pages just leave the count out.
//
// the token only ever lives here on the server; browsers just see the count.
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

// a visitor counts once per track per day (per dyno; it's a portfolio, not a bank)
const REPEAT_WINDOW: Duration = Duration::from_secs(24 * 60 * 60);

pub struct Counter {
    http: reqwest::Client,
    url: String,
    token: String,
    // keeps local dev from touching the real counts: dev:finds:hunt vs prod:finds:hunt
    prefix: String,
    seen: Mutex<HashMap<(String, &'static str), Instant>>,
}

pub fn track(name: &str) -> Option<&'static str> {
    match name {
        "hunt" => Some("hunt"),
        "ctf" => Some("ctf"),
        _ => None,
    }
}

impl Counter {
    pub fn from_env(prefix: &str) -> Option<Self> {
        let url = std::env::var("UPSTASH_REDIS_REST_URL").ok()?;
        let token = std::env::var("UPSTASH_REDIS_REST_TOKEN").ok()?;
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .ok()?;

        Some(Self {
            http,
            url: url.trim_end_matches('/').to_string(),
            token,
            prefix: prefix.to_string(),
            seen: Mutex::new(HashMap::new()),
        })
    }

    /// record a find (once per visitor per day) and return the total
    pub async fn found(&self, track: &'static str, visitor: &str) -> Option<u64> {
        let key = format!("{}:finds:{}", self.prefix, track);
        let command = if self.first_time(track, visitor) {
            "incr"
        } else {
            "get"
        };
        self.call(&format!("{}/{}", command, key)).await
    }

    fn first_time(&self, track: &'static str, visitor: &str) -> bool {
        let mut seen = self.seen.lock().unwrap();
        let now = Instant::now();
        seen.retain(|_, at| now.duration_since(*at) < REPEAT_WINDOW);
        seen.insert((visitor.to_string(), track), now).is_none()
    }

    // upstash's rest api: GET {url}/{command}/{args...} -> {"result": ...}
    async fn call(&self, path: &str) -> Option<u64> {
        let res = self
            .http
            .get(format!("{}/{}", self.url, path))
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| log::warn!("counter: {}", e))
            .ok()?;
        let body: serde_json::Value = res.json().await.ok()?;
        match &body["result"] {
            serde_json::Value::Number(n) => n.as_u64(),
            serde_json::Value::String(s) => s.parse().ok(),
            // a `get` before anyone has found it
            serde_json::Value::Null => Some(0),
            _ => None,
        }
    }
}
