use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
pub struct PgPoolWrapper{
  pub pool: PgPool,
}

pub async fn establish_connection() -> PgPool {  
  let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
  
  PgPoolOptions::new()
    .max_connections(100)
    .connect(&db_url)
    .await.expect("Unable to connect to Postgres")
}
