use std::fs;
use std::process::Command;
use std::time::Instant;

use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

use colored::*;
use serde_json::json;

use crate::socket::constants;
use crate::socket::message::TermMessage;

pub struct TermWebSocket {
    /// client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl TermWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second
    /// and checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(constants::HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > constants::CLIENT_TIMEOUT {
                println!("websocket client heartbeat failed, disconnecting");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for TermWebSocket {
    type Context = WebsocketContext<Self>;

    /// method is called on actor start, which begins the heartbeat proc
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        println!("started websocket!");
    }
}

impl Handler<TermMessage> for TermWebSocket {
    type Result = ();

    fn handle(&mut self, msg: TermMessage, ctx: &mut <Self as Actor>::Context) {
        println!("websocket <- terminal: {:?}", msg);

        match msg {
            TermMessage::Stdout(_) => {
                let json = serde_json::to_string(&msg);

                if let Ok(json) = json {
                    ctx.text(json);
                }
            }
            _ => eprintln!("invalid event - only stdout supported"),
        }
    }
}

pub fn read_faux_logs(p: String) -> String {
    let res = fs::read_to_string(p).unwrap();
    res
}

pub fn from_json(json: &str) -> Result<String, ()> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|_| {
        eprintln!("invalid json");
    })?;

    let list: &Vec<serde_json::Value> = value.as_array().ok_or_else(|| {
        eprintln!("must be an array");
    })?;

    match list
        .first()
        .ok_or_else(|| {
            eprintln!("empty array");
        })?
        .as_str()
        .ok_or_else(|| {
            eprintln!("type field is not a string");
        })? {
        "stdin" => {
            println!("STDIN VALUE -> {}", list.last().unwrap().to_string());

            match list.last() {
                Some(x) => {
                    let s: String = x.to_string().replace('"', "");
                    return Ok(s);
                }
                None => return Err(()),
            }
        }
        "stdout" => {
            println!("STDOUT VALUE -> {}", list.last().unwrap().to_string());

            match list.last() {
                Some(x) => {
                    let s: String = x.to_string().replace('"', "");
                    return Ok(s);
                }
                None => return Err(()),
            }
        }
        v => {
            eprintln!("error ocurred: {}", v);
            Err(())
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TermWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process ws messages
        println!("WS: {:?}", msg);

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }

            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }

            Ok(ws::Message::Text(text)) => {
                let m = String::from_utf8(Vec::from(&text[..])).unwrap();
                println!("M: {:?}", m);

                if m == "print_faux_logs" {
                    let faux_logs = read_faux_logs(String::from("static/assets/files/logs.txt"));
                    let data = json!({
                        "key": m,
                        "message": faux_logs,
                    });

                    ctx.text(data.to_string())
                } else if m.contains("\\r") {
                    ctx.text(format!("{}", "kathleenfrench@portfolio $ ".green().bold()))
                } else {
                    match from_json(&text) {
                        Ok(v) => ctx.text(v),
                        Err(_) => (),
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }

            _ => ctx.stop(),
        }
    }
}
