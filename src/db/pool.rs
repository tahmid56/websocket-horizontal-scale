use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pg_pool() -> PgPool {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}
