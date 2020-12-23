use anyhow::Result;
use uuid::Uuid;

use crate::ctx::RequestContext;
use crate::messages::request::RequestMsg;
use crate::messages::response::ResponseMsg;

#[derive(Debug)]
pub struct MessageHandler {
    conn_id: Uuid,
    chan_id: String,
    ctx: RequestContext,
}

impl MessageHandler {
    pub fn new(conn_id: Uuid, chan_id: String, ctx: RequestContext) -> Self {
        Self {
            conn_id,
            chan_id,
            ctx,
        }
    }

    pub const fn conn_id(&self) -> &Uuid {
        &self.conn_id
    }

    pub const fn chan_id(&self) -> &String {
        &self.chan_id
    }

    pub const fn ctx(&self) -> &RequestContext {
        &self.ctx
    }

    fn send_to_self(&self, msg: ResponseMsg) -> Result<()> {
        self.ctx()
            .connections()
            .send_connection(self.conn_id(), msg);
        Ok(())
    }

    fn send_to_channel(&self, msg: &ResponseMsg) -> Result<()> {
        self.ctx().connections().send_channel(self.chan_id(), msg);
        Ok(())
    }

    fn throw_to_self_channel_exception(&self, msg: &ResponseMsg) -> Result<()> {
        self.ctx()
            .connections()
            .throw_channel_exception(self.chan_id(), &[self.conn_id()], msg);
        Ok(())
    }

    pub fn on_open(&self) -> Result<()> {
        self.send_to_self(ResponseMsg::Connected {
            id: *self.conn_id(),
        })
    }

    pub fn on_close(&self) -> Result<()> {
        println!(
            "closing connection for [{}:{}]",
            self.conn_id(),
            self.chan_id()
        );
        Ok(())
    }

    pub fn on_message(&self, msg: RequestMsg) -> Result<()> {
        match msg {
            RequestMsg::Ping { v } => self.send_to_self(ResponseMsg::Pong { v }),
            msg => {
                println!("unhandled request message [{:?}]", msg);
                Ok(())
            }
        }
    }
}
