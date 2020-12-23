use actix_web::{web, HttpResponse};

use handlebars::Handlebars;

pub async fn about(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "location": "brooklyn, ny"
    });

    let body = hb.render("about", &data).unwrap();
    HttpResponse::Ok().body(body)
}
