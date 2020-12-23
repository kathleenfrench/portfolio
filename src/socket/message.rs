use actix::Message;
use libc::c_ushort;
use std::convert::TryFrom;

use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use serde_json;

use crate::socket::event::IO;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TermMessage {
    Stdin(IO),
    Stdout(IO),
    Resize { rows: c_ushort, cols: c_ushort },
}

impl Message for TermMessage {
    type Result = ();
}

impl TermMessage {
    pub fn from_json(json: &str) -> Result<Self, ()> {
        let value: serde_json::Value = serde_json::from_str(json).map_err(|_| {
            eprintln!("Invalid Terminado message: Invalid JSON");
        })?;

        let list: &Vec<serde_json::Value> = value.as_array().ok_or_else(|| {
            eprintln!("Invalid Terminado message: Needs to be an array!");
        })?;

        match list
            .first()
            .ok_or_else(|| {
                eprintln!("empty array!");
            })?
            .as_str()
            .ok_or_else(|| {
                eprintln!("type field not a string");
            })? {
            "stdin" => {
                if list.len() != 2 {
                    eprintln!("invalid length, stdin must equal 2");
                    return Err(());
                }

                Ok(TermMessage::Stdin(IO::from(list[1].as_str().ok_or_else(
                    || {
                        eprintln!("stdin must be a string");
                    },
                )?)))
            }
            "stdout" => {
                if list.len() != 2 {
                    eprintln!("invalid length, stdout must equal 2");
                    return Err(());
                }

                Ok(TermMessage::Stdout(IO::from(list[1].as_str().ok_or_else(
                    || {
                        eprintln!("stdout must be a string");
                    },
                )?)))
            }
            "set_size" => {
                if list.len() != 3 {
                    eprintln!("invalid length, set_size must equal 2");
                    return Err(());
                }

                let rows: u16 = u16::try_from(
                    list[1]
                        .as_u64()
                        .ok_or_else(|| eprintln!("first element must be an integer"))?,
                )
                .map_err(|_| eprintln!("set size rows out of range"))?;

                let cols: u16 = u16::try_from(
                    list[2]
                        .as_u64()
                        .ok_or_else(|| eprintln!("element 2 must be an integer"))?,
                )
                .map_err(|_| eprintln!("set size cols out of range"))?;

                Ok(TermMessage::Resize { rows, cols })
            }
            v => {
                eprintln!("unkonwn type: {:?}", v);
                Err(())
            }
        }
    }
}

impl Serialize for TermMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TermMessage::Stdin(stdin) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("stdin")?;
                seq.serialize_element(&String::from_utf8_lossy(stdin.0.as_ref()))?;
                seq.end()
            }
            TermMessage::Stdout(stdout) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("stdout")?;
                seq.serialize_element(&String::from_utf8_lossy(stdout.0.as_ref()))?;
                seq.end()
            }
            TermMessage::Resize { rows, cols } => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element("set_size")?;
                seq.serialize_element(rows)?;
                seq.serialize_element(cols)?;
                seq.end()
            }
        }
    }
}
