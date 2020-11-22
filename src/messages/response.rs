use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[allow(variant_size_differences)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResponseMsg {
  ServerError {
    cause: String,
    message: String,
  },
  Pong {
    v: i64
  },
  Connected {
    id: Uuid,
  }
}

impl ResponseMsg {
  pub fn from_bin(b: &[u8]) -> Result<Self> {
    bincode::deserialize(b).with_context(|| "cannot decode binary response")
  }

  pub fn to_bin(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).with_context(|| "cannot encode binary response")
  }

  pub fn from_json(s: &str) -> Result<Self> {
    serde_json::from_str(s).with_context(|| "cannot decode json response")
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string_pretty(&self).with_context(|| "cannot encode json response")
  }
}

/// TODO: add resume, responses, etc
#[derive(Debug, Serialize, Deserialize)]
pub enum WsResponseMsg {
  MessageCollection {
    messages: Box<Vec<ResponseMsg>>
  }
}