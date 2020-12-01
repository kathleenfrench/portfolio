use actix::Message;
// use io::{Read, Write};
// use std::io::{self};
// use futures::Future;
// use libc::c_ushort;
// use tokio_pty_process::PtyMaster;
// use std::{
//   pin::Pin,
//   task::{Context, Poll},
// };

use tokio_codec::{BytesCodec, Decoder};
type BytesMut = <BytesCodec as Decoder>::Item;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IO(pub BytesMut);

impl Message for IO {
  type Result = ();
}

impl AsRef<[u8]> for IO {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl From<String> for IO {
  fn from(s: String) -> Self {
    Self(s.into())
  }
}

impl From<&str> for IO {
  fn from(s: &str) -> Self {
    Self(s.into())
  }
}

impl From<actix_web::web::Bytes> for IO {
  fn from(b: actix_web::web::Bytes) -> Self {
    Self(b.as_ref().into())
  }
}

impl From<actix_web::web::BytesMut> for IO {
  fn from(b: actix_web::web::BytesMut) -> Self {
    Self(b.as_ref().into())
  }
}