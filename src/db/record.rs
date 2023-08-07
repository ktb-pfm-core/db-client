use tauri::command;
use tauri::State;
use serde::{Deserialize, Serialize};
// use sqlx::types::time::PrimitiveDateTime;
use super::client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  pub id: Option<i64>,
  pub project_name: Option<String>,
  pub service_name: Option<String>,
  pub api_name: Option<String>,
  pub image_tag: Option<String>,
  pub vu: Option<i16>,
  pub duration: Option<String>,
  pub tps: Option<f64>,
  pub error_rate: Option<f64>,
  pub rt_avg: Option<f64>,
  pub rt_min: Option<f64>,
  pub rt_max: Option<f64>,
  pub rt_p90: Option<f64>,
  pub rt_p95: Option<f64>,
  pub rt_p99: Option<f64>,
  pub is_cpu_below_request: Option<bool>,
  pub is_pipeline: Option<bool>,
  pub timestamp: Option<String>,
}

#[command]
pub async fn list_record(state: State<'_, client::PgPoolWrapper>) -> Result<Vec<Record>, String> {
  let rows: Vec<Record> = sqlx::query_as!(
    Record,
    r#"select * from public.records"#
  ).fetch_all(&state.pool)
    .await.expect("Unable to list Record");

  // println!("{:?}", rows);

  Ok(rows)
}

#[command]
pub async fn get_record(state: State<'_, client::PgPoolWrapper>, id: i64) -> Result<Record, String> {
  let row: Record = sqlx::query_as!(
    Record,
    r#"select * from public.records where id = ($1)"#,
    id
  ).fetch_one(&state.pool)
    .await.expect("Unable to get Record");

  // println!("{:?}", row);

  Ok(row)
}