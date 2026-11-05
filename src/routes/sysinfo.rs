// tiny bits of real server info for the terminal's `neofetch` and `who`
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use std::sync::LazyLock;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::counter::Counter;

static STARTED: LazyLock<Instant> = LazyLock::new(Instant::now);

/// call at startup so uptime counts from boot, not the first request
pub fn start_clock() {
    LazyLock::force(&STARTED);
}

/// GET /api/sysinfo
pub async fn sysinfo() -> HttpResponse {
    // heroku sets this with `heroku labs:enable runtime-dyno-metadata`
    let commit = std::env::var("HEROKU_SLUG_COMMIT")
        .or_else(|_| std::env::var("SOURCE_VERSION"))
        .ok()
        .map(|c| c.chars().take(7).collect::<String>());

    HttpResponse::Ok().json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "rustc": env!("BUILD_RUSTC"),
        "built": env!("BUILD_DATE"),
        "commit": commit,
        "uptime_secs": STARTED.elapsed().as_secs(),
    }))
}

#[derive(Deserialize)]
pub struct Checkin {
    id: String,
}

/// POST /api/who {"id": "..."}: check in, get the active tabs back.
/// sessions is null when there's no upstash configured
pub async fn who(
    body: web::Json<Checkin>,
    counter: web::Data<Option<Counter>>,
) -> Result<HttpResponse> {
    // ids are made by the page; keep them short and boring
    let id: String = body
        .id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .take(16)
        .collect();
    if id.is_empty() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let sessions = match counter.as_ref() {
        Some(c) => c.presence(&id, now).await,
        None => None,
    };
    let sessions = sessions.map(|list| {
        list.into_iter()
            .map(|(id, idle)| json!({ "id": id, "idle_secs": idle }))
            .collect::<Vec<_>>()
    });

    Ok(HttpResponse::Ok().json(json!({ "sessions": sessions })))
}
