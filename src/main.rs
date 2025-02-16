mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let pool = db::create_db_pool().await?;

  let res = sqlx::query!(
    "SELECT type_id FROM entity_types"
  )
  .fetch_all(&pool)
  .await?;

  for row in res {
    println!("{:?}", row.type_id);
  }

  Ok(())
}
