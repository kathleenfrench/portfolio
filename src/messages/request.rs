use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// payload from client -> server
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMsg {
    Ping { v: i64 },
}

impl RequestMsg {
    pub fn from_bin(b: &[u8]) -> Result<Self> {
        bincode::deserialize(b).with_context(|| "cannot decode binary request".to_string())
    }

    pub fn to_bin(&self) -> Result<Vec<u8>> {
        bincode::serialize(&self).with_context(|| "cannot encode binary request")
    }

    pub fn from_json(s: &str) -> Result<Self> {
        serde_json::from_str(s).with_context(|| format!("cannot decode json request: [{}]", s))
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).with_context(|| "cannot encode json request")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WsReqMessage {
    Cmd { name: String, action: String },
    BadRequest { cause: String, message: String },
    GoTo { link: String },
}
