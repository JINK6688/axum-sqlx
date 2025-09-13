use std::time::Duration;

use configure::CONFIG;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;
use tracing::info;

pub type PgPool = Pool<Postgres>;

pub static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_database() {
    info!("connecting to postgres");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(50))
        .connect(&CONFIG.database.get_url())
        .await
        .expect("connect database error");

    // Run migrations (requires `migrations/` at workspace root)
    // sqlx::migrate!("../migrations")
    //     .run(&pool)
    //     .await
    //     .expect("migrate database error");
    DB_POOL.set(pool).unwrap();
}

pub fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("Database pool is not initialized")
}
