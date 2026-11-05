// finder counts for the treasure hunt and the ctf, and who's on the site right
// now (`who`), kept in upstash redis via its rest api. optional: without
// UPSTASH_REDIS_REST_URL/_TOKEN (e.g. no .env locally) there's no counter, the
// pages leave the count out and `who` only shows you.
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

    /// `who`: mark this tab as active, and return the active tabs as
    /// (id, seconds since last seen). a tab is active if it checked in within
    /// the last two minutes. ids are random per tab; nothing else is stored
    pub async fn presence(&self, id: &str, now: u64) -> Option<Vec<(String, u64)>> {
        let key = format!("{}:who", self.prefix);
        let stale = now.saturating_sub(120).to_string();
        let now_s = now.to_string();
        let body = serde_json::json!([
            ["ZADD", key, now_s, id],
            ["ZREMRANGEBYSCORE", key, "-inf", stale],
            ["ZRANGE", key, "0", "-1", "WITHSCORES"],
            ["EXPIRE", key, "600"],
        ]);
        let res = self
            .http
            .post(format!("{}/pipeline", self.url))
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await
            .map_err(|e| log::warn!("presence: {}", e))
            .ok()?;
        let results: Vec<serde_json::Value> = res.json().await.ok()?;
        let flat = results.get(2)?["result"].as_array()?.clone();

        Some(
            flat.chunks(2)
                .filter_map(|pair| {
                    let id = pair.first()?.as_str()?.to_string();
                    let seen: u64 = pair.get(1)?.as_str()?.parse::<f64>().ok()? as u64;
                    Some((id, now.saturating_sub(seen)))
                })
                .collect(),
        )
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
