use actix_web::body::BoxBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{web, HttpResponse, Result};

use handlebars::Handlebars;
use serde_json::json;

// custom error handler for returning html error pages
pub fn error_handlers<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// error handler for 404s
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_right_body(),
    )))
}

// basic error handler
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // provide a fallback to a simple plain text response in case
    // an error occurs during the rendering of the error page
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let hb = request.app_data::<web::Data<Handlebars>>();

    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });

            match hb.render("error", &data) {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type("text/html; charset=utf-8")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
