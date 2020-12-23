use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::messages::response::ResponseMsg;

pub trait SendCallback: Sync + Send {
    fn send_message(&self, msg: ResponseMsg) -> ();
}

pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<Uuid, Box<dyn SendCallback>>>>,
    channels: Arc<RwLock<HashMap<String, HashSet<Uuid>>>>,
}

impl std::fmt::Debug for ConnectionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "connection manager [{}] connections, [{}] channels",
            &self
                .connections
                .read()
                .expect("cannot lock connections for read")
                .len(),
            &self
                .channels
                .read()
                .expect("cannot lock channels for read")
                .len()
        )
    }
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn connection_list(&self) -> Vec<Uuid> {
        let mut conns: Vec<Uuid> = self
            .connections
            .read()
            .expect("cannot lock connections")
            .keys()
            .copied()
            .collect();

        conns.sort();
        conns
    }

    pub fn channel_list(&self) -> Vec<(String, Vec<Uuid>)> {
        let mut chans: Vec<(String, Vec<Uuid>)> = self
            .channels
            .read()
            .expect("cannot lock channels")
            .iter()
            .map(|v| {
                let mut channel_ids: Vec<Uuid> = v.1.iter().copied().collect();
                channel_ids.sort();
                (v.0.clone(), channel_ids)
            })
            .collect();

        chans.sort();
        chans
    }

    pub fn remove(&self, key: &str, id: Uuid) {
        let _ = self
            .connections
            .write()
            .expect("cannot lock connections for write")
            .remove(&id);
        let mut chan = self
            .channels
            .write()
            .expect("cannot lock channels for write");

        match chan.get_mut(key) {
            Some(current_chan) => {
                if current_chan.contains(&id) {
                    let _ = current_chan.remove(&id);
                    println!(
                        "removed connection [{}] from [{}], [{}] connections remain",
                        id,
                        key,
                        current_chan.len()
                    );
                } else {
                    println!(
                        "could not find connection [{}] for [{}] in [{}] existing connections",
                        id,
                        key,
                        current_chan.len()
                    );
                }
            }
            None => {
                println!(
                    "tried to remove connection [{}] from [{}] which does not have any connections",
                    id, key
                );
            }
        }
    }

    pub fn add<F>(&self, key: &str, id: Uuid, f: Box<dyn SendCallback>) {
        let mut conns = self
            .connections
            .write()
            .expect("cannot lock connections for write");
        let _ = conns.insert(id, f);
        let mut chan = self
            .channels
            .write()
            .expect("cannot lock channels for write");

        match chan.get_mut(key) {
            Some(current_chan) => {
                let _ = current_chan.insert(id);
            }
            None => {
                let chan_set: HashSet<Uuid> = vec![id].into_iter().collect();
                let _ = chan.insert(key.into(), chan_set);
            }
        }
    }

    pub fn send_connection(&self, id: &Uuid, msg: ResponseMsg) {
        match &mut self
            .connections
            .read()
            .expect("cannot lock connections for read")
            .get(id)
        {
            Some(f) => {
                println!("sending message [{:?}] to connection [{}]", msg, &id);
                f.send_message(msg);
            }
            None => {
                println!("tried to send a message for a missing connection [{}]", &id);
            }
        }
    }

    pub fn send_channel(&self, key: &str, msg: &ResponseMsg) {
        self.throw_channel_exception(key, &[], msg)
    }

    pub fn throw_channel_exception(&self, key: &str, exclude: &[&Uuid], msg: &ResponseMsg) {
        match &mut self
            .channels
            .read()
            .expect("cannot lock channels for read")
            .get(key)
        {
            Some(current_chan) => {
                let current_chan_size = current_chan.len();
                let filtered: Vec<&Uuid> = current_chan
                    .iter()
                    .filter(|c| {
                        println!("{:?} / {} == {}", exclude, c, !exclude.contains(c));
                        !exclude.contains(c)
                    })
                    .collect();

                println!(
                    "sending message [{:?}] to [{}] using [{}] of [{}] connections",
                    msg,
                    key,
                    filtered.len(),
                    current_chan_size
                );

                let _: Vec<_> = filtered
                    .iter()
                    .map(|id| {
                        match self
                            .connections
                            .read()
                            .expect("cannot lock connections for read")
                            .get(id)
                        {
                            Some(f) => f.send_message(msg.clone()),
                            None => {
                                println!("could not send message");
                            }
                        }
                    })
                    .collect();
            }
            None => (),
        }
    }
}
