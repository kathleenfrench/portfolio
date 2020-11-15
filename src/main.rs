#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{guard, http::header, middleware, web, App, HttpServer};
use listenfd::ListenFd;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use handlebars::Handlebars;
use std::io;

mod handlers;
mod settings;
mod ws;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let cfg = CONFIG.clone();

    std::env::set_var("RUST_LOG", format!("actix_web={}", cfg.log.level));
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&cfg.ssl.key_file, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(&cfg.ssl.cert_file)
        .unwrap();

    // handlebars uses a repository for the compiled templates
    // this object must be shared between the application thread
    // and is passed to the application builder as an
    // atomic reference-counted pointer
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", format!("./{}", &cfg.static_paths.templates))
        .unwrap();

    let handlerbars_ref = web::Data::new(handlebars);

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain(&cfg.server.hostname)
                    .name(&cfg.server.session_key)
                    .secure(false),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::ORIGIN,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .wrap(handlers::error_handlers())
            .wrap(middleware::Logger::default())
            .app_data(handlerbars_ref.clone())
            .service(handlers::favicon)
            .service(
                Files::new("/assets", format!("{}/", &cfg.static_paths.assets))
                    .show_files_listing(),
            )
            .service(web::resource("/health").route(web::get().to(handlers::health_check)))
            .service(
                web::resource("/ws/").route(
                    web::get()
                        .guard(guard::Header("upgrade", "websocket"))
                        .to(ws::ws_index),
                ),
            )
            .service(handlers::index)
            .service(handlers::user)
            .service(handlers::about)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen_openssl(l, builder)?
    } else {
        // TODO: fix this to use a closure and avoid redundancy of re-cloning
        let cfg = CONFIG.clone();
        server.bind_openssl(
            format!("{}:{}", &cfg.server.hostname, &cfg.server.port),
            builder,
        )?
    };

    server.run().await
}
