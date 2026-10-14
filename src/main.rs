use actix_files::Files;
use actix_web::dev::Service;
use actix_web::http::header::{HeaderValue, CACHE_CONTROL};
use actix_web::{middleware, web, App, HttpServer};
use handlebars::{DirectorySourceOptions, Handlebars};
use std::io;
use std::sync::LazyLock;

mod cache;
mod handlers;
mod routes;
mod settings;

static CONFIG: LazyLock<settings::Settings> =
    LazyLock::new(|| settings::Settings::new().expect("config can't be loaded"));

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(format!(
        "portfolio={0},actix_web={0},actix_server={0}",
        CONFIG.log.level
    )))
    .init();

    // templates are compiled once at startup and shared across worker threads
    let mut handlebars = Handlebars::new();
    let mut template_opts = DirectorySourceOptions::default();
    template_opts.tpl_extension = ".html".to_owned();
    handlebars
        .register_templates_directory(&CONFIG.static_paths.templates, template_opts)
        .expect("templates can't be loaded");
    let handlebars = web::Data::new(handlebars);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .expect("PORT must be a number");

    log::info!("listening on http://{}:{}", CONFIG.server.host, port);

    HttpServer::new(move || {
        // the last `.wrap` is the outermost middleware
        App::new()
            .wrap(handlers::error_handlers())
            .wrap_fn(|req, srv| {
                let policy = cache::policy_for(req.path());
                let fut = srv.call(req);
                async move {
                    let mut res = fut.await?;
                    if let (Some(policy), true) = (policy, res.status().is_success()) {
                        res.headers_mut()
                            .insert(CACHE_CONTROL, HeaderValue::from_static(policy));
                    }
                    Ok(res)
                }
            })
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(handlebars.clone())
            .service(Files::new("/assets", &CONFIG.static_paths.assets).use_last_modified(true))
            .configure(routes::add_routes)
    })
    .bind((CONFIG.server.host.as_str(), port))?
    .run()
    .await
}
