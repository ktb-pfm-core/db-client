use app::db::{record,client};
use sqlx::PgPool;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let pool:PgPool = client::establish_connection().await;

  let report: Result<Vec<record::Record>, String> = record::list_record(pool, String::from("next"), String::from("aug-release")).await;

  println!("{:?}", report);
}