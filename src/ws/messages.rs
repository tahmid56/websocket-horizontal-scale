use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "create")]
    Create { user_id: String,room: String },

    #[serde(rename = "join")]
    Join { user_id: String, room: String },

    #[serde(rename = "leave")]
    Leave { user_id: String, room: String },

    #[serde(rename = "message")]
    Message { user_id: String, room: String, message: String },
}
