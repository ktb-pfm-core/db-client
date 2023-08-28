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
  Path((project, release, execution_type)): Path<(String, String, String)>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query_as!(
      Record,
      r#"select service,flow,flow_level_id,api_level_id,service_idx,api_idx,tag,vu,duration,tps,error_rate,rt_avg,rt_min,rt_max,rt_p90,rt_p95,rt_p99,is_cpu_below_request,resource_map,replica_map,cpu_utilization,cpu_request,cpu_limit,memory_utilization,memory_request,memory_limit,monitoring_db,timestamp,start_time,end_time from record.report where project = ($1) and release = ($2) and execution_type = ($3)"#,
      project,
      release,
      execution_type,
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
                "message": format!("Report with Project: {}, Release: {}, Execution Type: {} not found.", project, release, execution_type)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}
