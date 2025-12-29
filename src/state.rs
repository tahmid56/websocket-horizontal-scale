use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::ws::session::WsSession;
use actix::Addr;

pub type RoomName = String;
pub type ClientId = String;

#[derive(Default, Clone)]
pub struct ChatState {
    pub rooms: HashMap<RoomName, HashSet<ClientId>>,
    pub sessions: HashMap<ClientId, Addr<WsSession>>,
}

pub type SharedState = Arc<Mutex<ChatState>>;
