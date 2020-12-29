#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{http::header, middleware, web, App, HttpServer};

use handlebars::Handlebars;
use std::io;

mod handlers;
mod routes;
mod settings;
mod utils;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can't be loaded");
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", format!("actix_web={},actix_server={}", &CONFIG.log.level, &CONFIG.log.level));
    env_logger::init();

    // handlebars uses a repository for the compiled templates
    // this object must be shared between the application thread
    // and is passed to the application builder as an
    // atomic reference-counted pointer
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", format!("./{}", &CONFIG.static_paths.templates))
        .unwrap();

    let handlerbars_ref = web::Data::new(handlebars);

    let mut secure_cookie: bool = false;
    if &CONFIG.env.to_string() == "Prod" {
        secure_cookie = true;
    }

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain(&CONFIG.server.host)
                    .name(&CONFIG.server.session_key)
                    .path("/")
                    .secure(secure_cookie),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::ORIGIN,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CACHE_CONTROL,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .wrap(handlers::error_handlers())
            .wrap(middleware::Logger::default())
            .app_data(handlerbars_ref.clone())
            .service(utils::file_handler("/assets", &format!("{}/", &CONFIG.static_paths.assets)))
            .configure(|s| routes::add_routes(s))
    });

    let port: String = std::env::var("PORT").unwrap_or_else(|_| "3000".into());

    server.bind(format!("{}:{}", &CONFIG.server.host, &port))?.run().await
}
