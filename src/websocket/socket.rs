use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};

use anyhow::Result;
use std::time::Instant;
use std::fs;

use crate::messages::handlers::MessageHandler;
use crate::messages::response::ResponseMsg;
use crate::messages::request::RequestMsg;

use crate::websocket::constants::{HEARTBEAT_INTERVAL, CLIENT_TIMEOUT};
use crate::websocket::server::WsServerSender;
use crate::websocket::response::SendWsResponseMessage;

// ws connection
pub struct WebSocket {
  /// client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
  /// otherwise we drop connection.
  hb: Instant,
  binary: bool,
  handler: MessageHandler,
}

impl actix::Handler<SendWsResponseMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, m: SendWsResponseMessage, ctx: &mut Self::Context) {
        match self.send_ws(m.msg(), ctx) {
            Ok(_) => (),
            Err(e) => self.handle_err(&e.context(format!("error sending message [{:?}]", m.msg())), ctx)
        }
    }
}

impl WebSocket {
    // handler: MessageHandler
    pub fn new(binary: bool, handler: MessageHandler) -> Self {
        Self { 
            hb: Instant::now(),
            binary,
            handler,
        }
    }

    const fn handler(&self) -> &MessageHandler {
        &self.handler
    }

    fn handle_text(&self, c: String, _ctx: &mut WebsocketContext<Self>) -> Result<()> {
        let req = RequestMsg::from_json(&c)?;
        self.handler.on_message(req)
    }

    fn handle_binary(&self, bytes: bytes::Bytes, _ctx: &mut WebsocketContext<Self>) -> Result<()> {
        let b: &[u8] = bytes.as_ref();
        // let req = RequestMsg::from_bin(&b.to_vec())?;
        let req = RequestMsg::from_bin(&b.to_vec())?;
        self.handler.on_message(req)
    }

    fn handle_err(&self, e: &anyhow::Error, ctx: &mut WebsocketContext<Self>) {
        let msg = ResponseMsg::ServerError {
            cause: format!("{}", e),
            message: "could not handle message".into()
        };

        match self.send_ws(&msg, ctx) {
            Ok(_) => (),
            Err(e) => eprintln!("could not send server message: {}", e)
        }
    }

    fn send_ws(&self, resp: &ResponseMsg, ctx: &mut WebsocketContext<Self>) -> Result<()> {
        if self.binary {
            ctx.binary(resp.to_bin()?)
        } else {
            ctx.text(resp.to_json()?)
        }

        Ok(())
    }

    /// helper method that sends ping to client every second
    /// and checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("websocket client heartbeat failed, disconnecting");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// method is called on actor start, which begins the heartbeat proc
    fn started(&mut self, ctx: &mut Self::Context) {
        {
            let sender = Box::new(WsServerSender::new(ctx.address()));
            let connections = self.handler.ctx().connections();
            connections.add::<WsServerSender>(self.handler.chan_id(), *self.handler().conn_id(), sender);
        }

        match self.handler.on_open() {
            Ok(_) => (),
            Err(e) => eprintln!("could not open connection: {}", e)
        };
        // self.hb(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        match self.handler.on_close() {
            Ok(_) => (),
            Err(e) => eprintln!("could not process on_close: {}", e)
        };

        let conns = self.handler.ctx().connections();
        conns.remove(self.handler.chan_id(), *self.handler().conn_id());
        actix::Running::Stop
    }
}

pub fn read_faux_logs(p: String) -> String {
    let res = fs::read_to_string(p).unwrap();
    res
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process ws messages
        println!("WS: {:?}", msg);

        match msg {
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Text(text)) => match &self.handle_text(text, ctx) {
                Ok(_) => (),
                Err(e) => self.handle_err(e, ctx)
            },
            Ok(Message::Binary(bin)) => match self.handle_binary(bin, ctx) {
                Ok(_) => (),
                Err(e) => self.handle_err(&e, ctx)
            },
            _ => ()
            // Ok(ws::Message::Ping(msg)) => {
            //     self.hb = Instant::now();
            //     ctx.pong(&msg);
            // }

            // Ok(ws::Message::Pong(_)) => {
            //     self.hb = Instant::now();
            // }

            // Ok(ws::Message::Text(text)) => {
            //     let m = String::from_utf8(Vec::from(&text[..])).unwrap();
            //     println!("M: {:?}", m);
            //     if m == "print_faux_logs" {
            //         let faux_logs = read_faux_logs(String::from("static/assets/files/logs.txt"));
            //         let data = json!({
            //             "key": m,
            //             "message": faux_logs,
            //         });
            //         ctx.text(data.to_string())
            //     } else {
            //         if text.contains("stdin") {
            //             println!("stdin input: {}", text);
            //         }

            //         ctx.text(text)
            //     }
            // },
            // Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            // Ok(ws::Message::Close(reason)) => {
            //     ctx.close(reason);
            //     ctx.stop();
            // }

            // _ => ctx.stop(),
        }
    }
}