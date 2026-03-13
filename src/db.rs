use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn init_pool(database_url: &str) -> SqlitePool {
    std::fs::create_dir_all("data").expect("Failed to create data directory");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
