use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait Platform {
  // Platform-specific configuration
  type Config;
  // Platform-specific error
  type Error: std::error::Error;

  // Initialize the platform with config
  async fn new(config: Self::Config) -> Result<Self, Self::Error>
  where
    Self: Sized;

  // Start listening for events
  async fn start(&mut self, pool: SqlitePool) -> Result<(), Self::Error>;

  // Stop listening for events
  async fn stop(&mut self) -> Result<(), Self::Error>;
}
