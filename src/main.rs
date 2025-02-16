mod db;
mod models;

use models::{MetricDefinition, EntityType, MetricValue};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let pool = db::create_db_pool().await?;

  // Create a metric definition
  // MetricDefinition::create(
  //   &pool,
  //   "channel_vs_dm",
  //   "metrics.channel_vs_dm.name",
  //   "metrics.channel_vs_dm.description",
  //   EntityType::User
  // ).await?;

  // Update metric values
  MetricValue::update(
    &pool,
    "user123",
    EntityType::User,
    "channel_vs_dm",
    10.0,
    Some(5.0),
    None,
    Utc::now()
  ).await?;

  println!("Metric created and updated successfully!");

  Ok(())
}
