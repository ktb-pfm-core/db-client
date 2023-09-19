use std::sync::Arc;
use serde::Deserialize;
use serde_json::json;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::postgres::types::PgInterval;
use crate::{
  model::record::Record,
  state::state::AppState,
};

#[derive(Deserialize)]
pub struct GetReport {
    project: String,
    release: String,
    execution_type: String,
    duration_seconds: i64,
}

pub async fn health_checker_handler() -> impl IntoResponse {
  Json(json!({
    "status": "200",
    "message": "OK"
  }))
}

pub async fn get_report_handler(
  State(data): State<Arc<AppState>>,
  Json(payload): Json<GetReport>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query_as!(
      Record,
      r#"select service,flow,flow_level_id,api_level_id,service_idx,api_idx,api_method,api_path,tag,vu,cast(duration as string),tps,error_rate,rt_avg,rt_min,rt_max,rt_p90,rt_p95,rt_p99,is_cpu_below_request,resource_map,replica_map,cpu_utilization,cpu_request,cpu_limit,memory_utilization,memory_request,memory_limit,monitoring_db,timestamp,start_time,end_time from record.report where project = ($1) and release = ($2) and execution_type = ($3) and duration >= ($4)"#,
      payload.project,
      payload.release,
      payload.execution_type,
      PgInterval {
        days: 0,
        months: 0,
        microseconds: payload.duration_seconds * 1_000_000,
    },
    ).fetch_all(&data.db)
      .await;
    match result {
        Ok(rows) => {
            let res: serde_json::Value = serde_json::json!({"status": "success","data": serde_json::json!({
                "record": rows
            })});

            Ok(Json(res))
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Report with Project: {}, Release: {}, Execution Type: {} not found.", payload.project, payload.release, payload.execution_type)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}
