use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChatEventType {
    CreateRoom,
    JoinRoom,
    LeaveRoom,
    Message,
}

impl From<String> for ChatEventType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "create" => ChatEventType::CreateRoom,
            "join" => ChatEventType::JoinRoom,
            "leave" => ChatEventType::LeaveRoom,
            "message" => ChatEventType::Message,
            _ => {
                ChatEventType::Message
            }
        }
    }
}


use std::str::FromStr;

impl FromStr for ChatEventType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" => Ok(Self::CreateRoom),
            "join" => Ok(Self::JoinRoom),
            "leave" => Ok(Self::LeaveRoom),
            "message" => Ok(Self::Message),
            _ => Err(()),
        }
    }
}




#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ChatEvent {
    pub event_type: String,
    pub room: String,
    pub user_id: String,
    pub payload: Option<String>,
}
