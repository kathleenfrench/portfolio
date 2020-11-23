use std::time::{Duration, Instant};
use std::fs;

use actix::prelude::*;
use actix_web_actors::ws;

use serde_json::json;

use crate::socket::constants;

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
  type Context = ws::WebsocketContext<Self>;

  /// method is called on actor start, which begins the heartbeat proc
  fn started(&mut self, ctx: &mut Self::Context) {
      self.hb(ctx);
  }
}

pub fn read_faux_logs(p: String) -> String {
  let res = fs::read_to_string(p).unwrap();
  res
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
              } else {
                  if text.contains("stdin") {
                      println!("stdin input: {}", text);
                  }

                  ctx.text(text)
              }
          },
          Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
          Ok(ws::Message::Close(reason)) => {
              ctx.close(reason);
              ctx.stop();
          }

          _ => ctx.stop(),
      }
  }
}

