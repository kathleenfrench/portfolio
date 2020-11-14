#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate actix_web;

use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use listenfd::ListenFd;

use handlebars::Handlebars;
use std::io;

mod handlers;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    // handlebars uses a repository for the compiled templates
    // this object must be shared between the application thread
    // and is passed to the application builder as an
    // atomic reference-counted pointer
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    
    let handlerbars_ref = web::Data::new(handlebars);

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(handlers::error_handlers())
            // enable the logger
            .wrap(middleware::Logger::default())
            // allow visitor to see index of assets at /assets
            .service(Files::new("/assets", "static/assets/").show_files_listing())
            .service(Files::new("/dist", "dist/").show_files_listing())
            // [note]: you can serve a tree of static files at the web root
            // and specify the index file
            // the root path should always be defined as the last
            // item, the paths are resolved in the order they are 
            // defined. if this would be placed before the /assets
            // path, then the service for the static assets would
            // never be reached
            .service(Files::new("/favicon.ico", "./static/favicon.ico"))
            .route("/health", web::get().to(handlers::health_check))
            .app_data(handlerbars_ref.clone())
            .service(handlers::index)
            .service(handlers::user)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}