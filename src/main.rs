mod db;
mod ws;
mod state;
mod redis;
mod event;
mod events;
mod bootstrap;

use actix_web::{App, HttpServer, web};
use ::redis::Client;
use state::ChatState;
use std::sync::{Arc, Mutex};
use crate::bootstrap::bootstrap;

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn run_migrations(pool: &PgPool) {
    MIGRATOR.run(pool).await.expect("Migration failed");
}
use sqlx::PgPool;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(Mutex::new(ChatState::default()));
    let db = db::pool::create_pg_pool().await;
    let redis = Client::open("redis://redis:6379").unwrap();
    bootstrap(&redis, &db, &state).await;
    {
        let redis_clone = redis.clone();
        let state_clone = state.clone();
        tokio::spawn(async move {
            redis::subscriber::start_subscriber(redis_clone, state_clone)
                .await
                .expect("Redis subscriber failed");
        });
    }
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(redis.clone()))
            .route("/ws", web::get().to(ws::handler::ws_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
