use actix_files::NamedFile;
use actix_http::{body::Body, Response};
use actix_session::Session;
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{web, HttpResponse, HttpRequest, Error, Result};
use actix_web_actors::ws;

use uuid::Uuid;

// use crate::ws::TermWebSocket;
use crate::messages::handlers::MessageHandler;
use crate::ctx::request_context;

use crate::websocket::socket::WebSocket;
use handlebars::Handlebars;

#[get("/ws/")]
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let bin = match r.query_string() {
        x if x.contains("t=b") => true,
        _ => false
    };

    let id = Uuid::new_v4();
    let handler = MessageHandler::new(id, r.path().to_string(), );
    let res = ws::start(WebSocket::new(binary, handler), &r, stream);
    println!("{:?}", res.as_ref().unwrap());
    res
}