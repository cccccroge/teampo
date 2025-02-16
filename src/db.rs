use sqlx::SqlitePool;
use std::error::Error;
use dotenv::dotenv;

pub async fn create_db_pool() -> Result<SqlitePool, Box<dyn Error>> {
    // load .env file
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
