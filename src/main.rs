use app::db::{record,client};
use sqlx::PgPool;
use tokio::runtime::Runtime;

fn main() {
  let pool:PgPool = Runtime::new().unwrap().block_on(client::establish_connection());

  Runtime::new().unwrap().block_on(record::list_record(pool, String::from("next"), String::from("aug-release")));
}