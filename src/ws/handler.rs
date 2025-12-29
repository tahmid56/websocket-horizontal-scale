use actix_web::{HttpRequest, HttpResponse, web, Error};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{
    ws::session::WsSession,
    state::SharedState,
};

use sqlx::PgPool;

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<SharedState>,
    db: web::Data<PgPool>,
    redis: web::Data<redis::Client>
) -> Result<HttpResponse, Error> {
    let session = WsSession {
        id: Uuid::new_v4().to_string(),
        state: state.get_ref().clone(),
        db_pool: db.get_ref().clone(),
        redis: redis.get_ref().clone(),
    };

    ws::start(session, &req, stream)
}
