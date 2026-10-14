use actix_web::HttpResponse;

pub async fn check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
