use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Result};

use handlebars::Handlebars;

pub async fn index(session: Session, hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    let mut visit_count = 1;

    if let Some(count) = session.get::<i32>("visit_count")? {
        println!("visit_count: {}", count);
        visit_count = count + 1;
    }

    session.set("visit_count", visit_count)?;

    let data = json!({
        "name": "kathleenfrench.co"
    });

    let body = hb.render("index", &data).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body))
}
