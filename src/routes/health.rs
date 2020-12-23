use actix_web::HttpResponse;

pub fn check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
