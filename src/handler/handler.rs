use std::sync::Arc;
use serde_json::json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
  model::record::Record,
  state::state::AppState,
};


pub async fn health_checker_handler() -> impl IntoResponse {
  Json(json!({
    "status": "200",
    "message": "OK"
  }))
}

pub async fn get_report_handler(
  Path((project, release)): Path<(String, String)>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query_as!(
      Record,
      r#"select execution_type,service,flow,tag,vu,duration,tps,error_rate,rt_avg,rt_min,rt_max,rt_p90,rt_p95,rt_p99,is_cpu_below_request,resource_map,replica_map,cpu_utilization,cpu_request,cpu_limit,memory_utilization,memory_request,memory_limit,timestamp,start_time,end_time from record.report where project = ($1) and release = ($2)"#,
      project,
      release,
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
                "message": format!("Report with Project: {} and Release: {} not found.", project, release)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}
