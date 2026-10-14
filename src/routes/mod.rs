use actix_web::web::{get, resource, ServiceConfig};

mod favicon;
mod health;
mod home;
mod treasure;

pub fn add_routes(s: &mut ServiceConfig) {
    s.service(
        resource("/health")
            .name("health")
            .route(get().to(health::check)),
    )
    .service(resource("/").name("home").route(get().to(home::index)))
    .service(
        resource("/favicon.ico")
            .name("favicon")
            .route(get().to(favicon::favicon)),
    )
    .service(
        resource("/dr9lrf26db8ori9")
            .name("treasure")
            .route(get().to(treasure::find)),
    );
}
