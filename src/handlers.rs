use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{web, HttpResponse, Result};
use actix_files::NamedFile;
use actix_session::Session;
use actix_utils::mpsc;

use handlebars::Handlebars;

#[get("/favicon")]
async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

#[get("/")]
pub async fn index(session: Session, hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION VALUE: {}", count);
        counter = count + 1;
    }

    session.set("counter", counter)?;

    let data = json!({
        "name": "kathleenfrench.co"
    });

    let body = hb.render("index", &data).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8").body(body))
}

#[get("/{user}/{data}")]
pub async fn user(
    hb: web::Data<Handlebars<'_>>,
    web::Path(info): web::Path<(String, String)>,
) -> HttpResponse {
    let data = json!({
        "user": info.0,
        "data": info.1,
    });

    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
}

// a health check endpoint
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// custom error handler for returning html error pages
pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// error handler for 404s
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// basic error handler
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    // provide a fallback to a simple plain text response in case
    // an error occurs during the rendering of the error page
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());

    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });

            let body = hb.render("error", &data);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn health_check_succss() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}