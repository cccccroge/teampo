use sqlx::SqlitePool;
use std::error::Error;
use chrono::{DateTime, Utc};

// Enum for entity types
#[derive(Debug)]
pub enum EntityType {
  User,
  Thread,
}

impl EntityType {
  pub fn as_str(&self) -> &'static str {
    match self {
      EntityType::User => "user",
      EntityType::Thread => "thread",
    }
  }
}

// Struct for metric definition
#[derive(Debug)]
pub struct MetricDefinition {
  pub metric_id: String,
  pub name_key: String,
  pub description_key: String,
  pub category: EntityType,
}

// Struct for metric value
pub struct MetricValue {
  pub entity_id: String,
  pub entity_type: EntityType,
  pub metric_id: String,
  pub value1: f64,
  pub value2: Option<f64>,
  pub value3: Option<f64>,
  pub period_start: DateTime<Utc>,
}

impl MetricDefinition {
  pub async fn create(
    pool: &SqlitePool,
    metric_id: &str,
    name_key: &str,
    description_key: &str,
    category: EntityType,
  ) -> Result<(), Box<dyn Error>> {
    let category_string = category.as_str();
    sqlx::query!(
      r#"
      INSERT INTO metric_definitions (metric_id, name_key,
      description_key, category)
      VALUES (?, ?, ?, ?)
      "#,
      metric_id,
      name_key,
      description_key,
      category_string,
    )
    .execute(pool)
    .await?;

    Ok(())
  }
}

impl MetricValue {
  pub async fn update(
    pool: &SqlitePool,
    entity_id: &str,
    entity_type: EntityType,
    metric_id: &str,
    value1: f64,
    value2: Option<f64>,
    value3: Option<f64>,
    period_start: DateTime<Utc>,
  ) -> Result<(), Box<dyn Error>> {
    let entity_type_str = entity_type.as_str();
    sqlx::query!(
      r#"
      INSERT INTO metric_values (entity_id, entity_type,
      metric_id, value1, value2, value3, period_start)
      VALUES (?, ?, ?, ?, ?, ?, ?)
      ON CONFLICT (entity_id, entity_type, metric_id)
      DO UPDATE SET value1 = ?, value2 = ?, value3 =?,
      period_start = ?
      "#,
      entity_id,
      entity_type_str,
      metric_id,
      value1,
      value2,
      value3,
      period_start,
      value1,
      value2,
      value3,
      period_start,
    )
    .execute(pool)
    .await?;

    Ok(())
  }
}
