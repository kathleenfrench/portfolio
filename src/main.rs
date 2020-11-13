#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_files::Files;
use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};

use handlebars::Handlebars;

use std::io;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "name": "kathleenfrench.co"
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/{user}/{data}")]
async fn user(
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

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // handlebars uses a repository for the compiled templates
    // this object must be shared between the application thread
    // and is passed to the application builder as an
    // atomic reference-counted pointer
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    
    let handlerbars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            // enable the logger
            .wrap(middleware::Logger::default())
            // allow visitor to see index of assets at /assets
            .service(Files::new("/assets", "static/assets/").show_files_listing())
            // serve a tree of static files at the web root
            // and specify the index file
            // the root path should always be defined as the last
            // item, the paths are resolved in the order they are 
            // defined. if this would be placed before the /assets
            // path,t hen the service for the static assets would
            // never be reached
            // .service(Files::new("/", "./static/root/").index_file("index.html"))
            .app_data(handlerbars_ref.clone())
            .service(index)
            .service(user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// custom error handler for returning html error pages
fn error_handlers() -> ErrorHandlers<Body> {
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