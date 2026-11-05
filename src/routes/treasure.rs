use actix_web::{error, web, HttpRequest, HttpResponse, Result};

use handlebars::Handlebars;
use serde_json::json;

use crate::counter::{self, Counter};

pub async fn find(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    page(&hb, "treasure")
}

/// the ctf's secret page (`goto kefahuchi`)
pub async fn event_horizon(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    page(&hb, "event_horizon")
}

fn page(hb: &Handlebars, template: &str) -> Result<HttpResponse> {
    let body = hb
        .render(template, &json!({}))
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

/// POST /api/found/{hunt|ctf}: count a find, return {"count": n}. count is
/// null when there's no counter configured (or upstash is unreachable)
pub async fn found(
    req: HttpRequest,
    track: web::Path<String>,
    counter: web::Data<Option<Counter>>,
) -> Result<HttpResponse> {
    let track = counter::track(&track).ok_or_else(|| error::ErrorNotFound("no such track"))?;

    // behind cloudflare, the visitor's ip is in CF-Connecting-IP
    let visitor = req
        .headers()
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .or_else(|| req.connection_info().realip_remote_addr().map(String::from))
        .unwrap_or_default();

    let count = match counter.as_ref() {
        Some(c) => c.found(track, &visitor).await,
        None => None,
    };

    Ok(HttpResponse::Ok().json(json!({ "count": count })))
}
