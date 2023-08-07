use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn establish_connection() -> PgPool {  
  let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
  
  PgPoolOptions::new()
    .max_connections(100)
    .acquire_timeout(Duration::new(30, 0))
    .connect(&db_url)
    .await.expect("Unable to connect to Postgres")
}
