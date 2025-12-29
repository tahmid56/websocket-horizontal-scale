use std::str::FromStr;

use crate::{
    event::{ChatEvent, ChatEventType},
    state::SharedState,
    ws::session::WsMessage,
};

pub async fn handle_event(event: ChatEvent, state: &SharedState) {
    let mut state = state.lock().unwrap();
    let event_type = ChatEventType::from_str(&event.event_type)
    .expect("invalid event type");

    match event_type {
        ChatEventType::CreateRoom => {
            state.rooms.entry(event.room).or_default();
        }

        ChatEventType::JoinRoom => {
            state.rooms
                .entry(event.room)
                .or_default()
                .insert(event.user_id.clone());
        }

        ChatEventType::LeaveRoom => {
            if let Some(users) = state.rooms.get_mut(&event.room) {
                users.remove(&event.user_id);
            }
        }

        ChatEventType::Message => {
            if let Some(users) = state.rooms.get(&event.room) {
                for user in users {
                    if let Some(addr) = state.sessions.get(user) {
                        addr.do_send(WsMessage(
                            event.payload.clone().unwrap_or_default(),
                        ));
                    }
                }
            }
        }
    }
}
