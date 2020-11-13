use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
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
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}