use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;

use crate::socket::ws::TermWebSocket;

pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(TermWebSocket::new(), &r, stream);
    println!("{:?}", res.as_ref().unwrap());
    res
}
