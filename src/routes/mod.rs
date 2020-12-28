use actix_web::web::{get, resource, ServiceConfig};

mod health;
mod home;
mod static_files;
mod treasure;

pub fn add_routes(s: &mut ServiceConfig) {
    let _ = s
        .service(
            resource("/health")
                .name("health")
                .route(get().to(health::check)),
        )
        .service(resource("/").name("home").route(get().to(home::index)))
        .service(
            resource("/favicon.ico")
                .name("favicon")
                .route(get().to(static_files::favicon)),
        )
        .service(
            resource("/dr9lrf26db8ori9")
                .name("treasure")
                .route(get().to(treasure::find)),
        );
}
