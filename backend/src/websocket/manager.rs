use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::info;
use actix::prelude::*;

pub struct ConnectionManager {
    users: HashMap<String, Vec<actix::prelude::Recipient<ServerMessage>>>,
}

pub struct ServerMessage {
    pub message: String,
}

impl Message for ServerMessage {
    type Result = ();
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn connect(&mut self, user_id: String, addr: actix::prelude::Recipient<ServerMessage>) {
        info!("User connected: {}", user_id);
        self.users.entry(user_id).or_insert_with(Vec::new).push(addr);
    }

    pub fn disconnect(&mut self, user_id: &str, addr: &actix::prelude::Recipient<ServerMessage>) {
        info!("User disconnected: {}", user_id);
        if let Some(addrs) = self.users.get_mut(user_id) {
            addrs.retain(|a| a != addr);
            if addrs.is_empty() {
                self.users.remove(user_id);
            }
        }
    }

    pub fn send_to_user(&mut self, user_id: &str, message: String) {
        if let Some(addrs) = self.users.get(user_id) {
            for addr in addrs {
                let _ = addr.do_send(ServerMessage {
                    message: message.clone(),
                });
            }
            info!("Sent message to user: {}", user_id);
        }
    }

    pub fn broadcast_message(&self, message: String) {
        info!("Broadcasting to {} users", self.users.len());
    }

    pub fn is_user_online(&self, user_id: &str) -> bool {
        self.users.contains_key(user_id)
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
