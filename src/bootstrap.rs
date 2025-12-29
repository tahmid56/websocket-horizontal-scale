use crate::{
    redis::lock::{try_acquire_lock, release_lock},
    events::replay::replay_events,
    state::SharedState,
};
use redis::Client;
use sqlx::PgPool;

pub async fn bootstrap(
    redis: &Client,
    db: &PgPool,
    state: &SharedState,
) {
    const LOCK_KEY: &str = "chat:bootstrap:lock";

    let acquired = try_acquire_lock(redis, LOCK_KEY, 30)
        .await
        .unwrap_or(false);

    if !acquired {
        println!("Another instance is bootstrapping.");
        return;
    }

    println!("Bootstrapping state from DB...");
    replay_events(state, db).await;

    release_lock(redis, LOCK_KEY).await.ok();
}
