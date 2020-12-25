use actix_web::{web, HttpResponse};

use handlebars::Handlebars;

pub async fn find(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "treasure": "helloooooo",
        "gold": "gold",
    });

    let body = hb.render("treasure", &data).unwrap();
    HttpResponse::Ok().body(body)
}
