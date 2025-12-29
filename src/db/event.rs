use sqlx::{PgPool};
use uuid::Uuid;
use crate::event::ChatEvent;

pub async fn log_event(
    pool: &sqlx::PgPool,
    room: &str,
    user_id: String,
    event_type: &str,
    payload: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO events (id, room, user_id, event_type, payload)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(Uuid::new_v4())
    .bind(room)
    .bind(user_id)
    .bind(event_type)
    .bind(payload)
    .execute(pool)
    .await?;

    Ok(())
}




pub async fn get_all_events(
    pool: &sqlx::PgPool,
) -> Result<Vec<ChatEvent>, sqlx::Error> {
    let events = sqlx::query_as::<_, ChatEvent>(
        r#"
        SELECT room, user_id, event_type, payload
        FROM events
        ORDER BY created_at ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(events)
}
