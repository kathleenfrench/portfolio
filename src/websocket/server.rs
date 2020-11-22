use actix::Addr;

use crate::websocket::response::SendWsResponseMessage;
use crate::websocket::socket::WebSocket;
use crate::messages::response::ResponseMsg;
use crate::conn::SendCallback;

pub struct WsServerSender {
  addr: Addr<WebSocket>
}

impl WsServerSender {
  pub fn new(addr: Addr<WebSocket>) -> Self {
      Self { addr }
  }
}

impl SendCallback for WsServerSender {
  fn send_message(&self, msg: ResponseMsg) {
      self.addr.do_send(SendWsResponseMessage { msg });
  }
}