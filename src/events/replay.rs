use crate::{
    db::event::get_all_events,
    events::handler::handle_event,
    state::SharedState,
};

pub async fn replay_events(state: &SharedState, db: &sqlx::PgPool) {
    let events = get_all_events(db).await.unwrap_or_default();

    for event in events {
        handle_event(event, state).await;
    }
}
