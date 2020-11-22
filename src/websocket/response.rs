use crate::messages::response::ResponseMsg;

#[derive(Debug)]
pub struct SendWsResponseMessage {
    pub msg: ResponseMsg,
}

impl SendWsResponseMessage {
    pub const fn msg(&self) -> &ResponseMsg {
        &self.msg
    }
}

impl actix::Message for SendWsResponseMessage {
    type Result = ();
}