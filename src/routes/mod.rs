use actix_web::web::{get, post, resource, ServiceConfig};

mod favicon;
mod health;
mod home;
pub mod sysinfo;
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
    )
    .service(
        resource("/tract-02732f49")
            .name("event_horizon")
            .route(get().to(treasure::event_horizon)),
    )
    .service(
        resource("/api/found/{track}")
            .name("found")
            .route(post().to(treasure::found)),
    )
    .service(
        resource("/api/sysinfo")
            .name("sysinfo")
            .route(get().to(sysinfo::sysinfo)),
    )
    .service(
        resource("/api/who")
            .name("who")
            .route(post().to(sysinfo::who)),
    );
}
