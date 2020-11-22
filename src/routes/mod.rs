use actix_web::web::{get, resource, ServiceConfig};

mod health;
mod user;
mod about;
mod home;
mod static_files;

pub fn add_routes(s: &mut ServiceConfig) {
  let _ = s
    .service(resource("/health").name("health").route(get().to(health::check)))
    .service(resource("/{user}/{data}").name("user").route(get().to(user::get_user)))
    .service(resource("/about").name("about").route(get().to(about::about)))
    .service(resource("/").name("home").route(get().to(home::index)))
    .service(resource("/favicon.ico").name("favicon").route(get().to(static_files::favicon)));
}