use crate::event::{ChatEvent, ChatEventType};
use crate::redis::publisher::publish_event;
use crate::{db::event::log_event, state::SharedState};
use actix::AsyncContext;
use actix::{Actor, Context, Handler, Message, StreamHandler};
use actix_web_actors::ws;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

pub struct WsSession {
    pub id: String,
    pub state: SharedState,
    pub db_pool: sqlx::PgPool,
    pub redis: redis::Client,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.state
            .lock()
            .unwrap()
            .sessions
            .insert(self.id.clone(), ctx.address());
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        let mut state = self.state.lock().unwrap();
        state.sessions.remove(&self.id);

        for users in state.rooms.values_mut() {
            users.remove(&self.id);
        }
    }
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

use crate::ws::messages::ClientMessage;

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let text = match msg {
            Ok(ws::Message::Text(t)) => t,
            _ => return,
        };

        let parsed: ClientMessage = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(_) => return,
        };

        let mut state = self.state.lock().unwrap();

        match parsed {
            ClientMessage::Create { user_id, room } => {
                state.rooms.entry(room.clone()).or_default();
                let event = ChatEvent {
                    event_type: "create".to_string(),
                    room: room.clone(),
                    user_id: user_id.clone(),
                    payload: None,
                };
                let db_pool = self.db_pool.clone();
                let redis = self.redis.clone();
                actix::spawn(async move {
                    log_event(&db_pool, &room, user_id, "create", None)
                        .await
                        .ok();
                    publish_event(&redis, &event);
                });
            }

            ClientMessage::Join { user_id, room } => {

                let event = ChatEvent {
                    event_type: "join".to_string(),
                    room: room.clone(),
                    user_id: user_id.clone(),
                    payload: None,
                };
                state
                    .rooms
                    .entry(room.clone())
                    .or_default()
                    .insert(self.id.clone());
                let db_pool = self.db_pool.clone();
                let redis = self.redis.clone();
                actix::spawn(async move {
                    log_event(&db_pool, &room, user_id, "join", None).await.ok();
                    publish_event(&redis, &event);
                });
            }

            ClientMessage::Leave { user_id, room } => {
                if let Some(r) = state.rooms.get_mut(&room) {
                    let event = ChatEvent {
                        event_type: "leave".to_string(),
                        room: room.clone(),
                        user_id: user_id.clone(),
                        payload: None,
                    };
                    r.remove(&self.id);
                    let db_pool = self.db_pool.clone();
                    let redis = self.redis.clone();
                    actix::spawn(async move {
                        log_event(&db_pool, &room, user_id, "remove", None)
                            .await
                            .ok();
                        publish_event(&redis, &event);
                    });
                }
            }

            ClientMessage::Message {
                user_id,
                room,
                message,
            } => {
                if let Some(users) = state.rooms.get(&room) {
                    for user in users {
                        if let Some(addr) = state.sessions.get(user) {
                            addr.do_send(WsMessage(format!("[{}] {}", user_id, message)));
                        }
                    }
                }

                let db_pool = self.db_pool.clone();
                let redis = self.redis.clone();

                let event = ChatEvent {
                    event_type: "message".to_string(),
                    room: room.clone(),
                    user_id: user_id.clone(),
                    payload: Some(message.clone()),
                };

                actix::spawn(async move {
                    log_event(&db_pool, &room, user_id, "message", Some(&message))
                        .await
                        .ok();

                    if let Err(e) = publish_event(&redis, &event) {
                        eprintln!("publish failed: {e}");
                    }
                });
            }
        }
    }
}
