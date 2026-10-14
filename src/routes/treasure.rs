use actix_web::{error, web, HttpResponse, Result};

use handlebars::Handlebars;
use serde_json::json;

pub async fn find(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    let data = json!({
        "treasure": "helloooooo",
        "gold": "gold",
    });

    let body = hb
        .render("treasure", &data)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}
