use std::sync::Arc;
use app::{db::client, route::cors::new_cors, route::router::create_router, state::state::AppState};
use sqlx::PgPool;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let pool:PgPool = client::establish_connection().await;
  let cors = new_cors();
  let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

  println!("Server started successfully.");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
