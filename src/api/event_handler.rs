// use std::collections::HashMap;
// use std::time::Duration;
// use axum::extract::State;
// use axum::http::StatusCode;
// use axum::Json;
// use serde::Deserialize;
// use serde_dynamo::{from_item, to_item};
// use tracing::{error, info};
// use crate::{get_current_timestamp, AppState};
// use crate::errors::AppError;
// use crate::models::{JobConfig, JobRun};
// use crate::notification::send_alert;
// 
// #[derive(Deserialize)]
// struct JobStartEvent {
//     job_name: String,
//     run_id: String,
// }
// 
// #[derive(Deserialize)]
// struct StageCompleteEvent {
//     run_id: String,
//     stage_name: String,
// }
// 
// pub async fn start_job(
//     State(state): State<AppState>,
//     Json(payload): Json<JobStartEvent>,
// ) -> Result<StatusCode, AppError> {
//     info!("Starting job: {} (Run: {})", payload.job_name, payload.run_id);
//     let start_time = get_current_timestamp();
// 
//     // 1. Get the JobConfig from DynamoDB
//     let config_item = state.db.get_item()
//         .table_name(&state.config_table)
//         .key("job_name", payload.job_name.clone().into())
//         .send()
//         .await?;
// 
//     let config: JobConfig = match config_item.item() {
//         Some(item) => from_item(item.clone())?,
//         None => return Err(AppError::NotFound(payload.job_name)),
//     };
// 
//     // 2. Create and save the initial JobRun
//     let job_run = JobRun {
//         run_id: payload.run_id.clone(),
//         job_name: payload.job_name.clone(),
//         start_time,
//         completed_stages: HashMap::new(),
//         is_active: true,
//     };
//     state.db.put_item()
//         .table_name(&state.run_table)
//         .item(to_item(job_run)?)
//         .send()
//         .await?;
// 
//     // 3. Spawn alert-checker tasks for each stage
//     let mut alert_tasks = Vec::new();
//     for stage in config.stages {
//         let task_state = state.clone(); // Clone state for the async task
//         let run_id = payload.run_id.clone();
//         let job_name = payload.job_name.clone();
//         let stage_name = stage.name.clone();
// 
//         info!(
//             "Spawning checker for stage '{}' ({}s limit)",
//             stage_name, stage.relative_time_limit_secs
//         );
// 
//         let handle = tokio::spawn(async move {
//             // Wait for the deadline
//             tokio::time::sleep(Duration::from_secs(stage.relative_time_limit_secs)).await;
// 
//             // Woke up! Now check if the stage was *actually* completed.
//             match is_stage_complete(&task_state, &run_id, &stage_name).await {
//                 Ok(true) => {
//                     // Completed on time, no alert needed.
//                     info!("Stage '{}' completed on time for run '{}'.", stage_name, run_id);
//                 }
//                 Ok(false) => {
//                     // DEADLINE MISSED!
//                     send_alert(
//                         &format!("ALERT: Job '{}' (Run: {}) MISSED deadline for stage '{}'!",
//                                  job_name, run_id, stage_name)
//                     );
//                 }
//                 Err(e) => {
//                     error!("Failed to check stage status for run '{}': {}", run_id, e);
//                 }
//             }
//         });
//         alert_tasks.push(handle);
//     }
// 
//     // 4. Store the task handles so we can cancel them later
//     state.active_tasks.insert(payload.run_id, alert_tasks);
//     Ok(StatusCode::ACCEPTED)
// }
// 
// pub async fn log_stage_complete(
//     State(state): State<AppState>,
//     Json(payload): Json<StageCompleteEvent>,
// ) -> Result<StatusCode, AppError> {
//     info!("Logging stage complete: {} (Run: {})", payload.stage_name, payload.run_id);
//     let completion_time = get_current_timestamp();
// 
//     // 1. Atomically update the JobRun in DynamoDB
//     let update_result = state.db.update_item()
//         .table_name(&state.run_table)
//         .key("run_id", payload.run_id.clone().into())
//         .update_expression("SET completed_stages.#stage = :time")
//         .expression_attribute_names("#stage", payload.stage_name.clone())
//         .expression_attribute_values(":time", completion_time.into())
//         // Return the *entire* updated item
//         .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
//         .send()
//         .await?;
// 
//     let updated_item = update_result.attributes().ok_or(AppError::DynamoError)?;
//     let updated_run: JobRun = from_item(updated_item.clone())?;
// 
//     // 2. Check if the job is now fully complete
//     // We need the config for this.
//     let config_item = state.db.get_item()
//         .table_name(&state.config_table)
//         .key("job_name", updated_run.job_name.clone().into())
//         .send()
//         .await?;
// 
//     let config: JobConfig = match config_item.item() {
//         Some(item) => from_item(item.clone())?,
//         None => return Err(AppError::NotFound(updated_run.job_name)),
//     };
// 
//     // Check if all configured stages are present in the completed_stages map
//     let all_stages_complete = config.stages.iter()
//         .all(|s| updated_run.completed_stages.contains_key(&s.name));
// 
//     if all_stages_complete {
//         info!("Job run {}/{} is fully complete!", updated_run.job_name, updated_run.run_id);
// 
//         // 3. Mark job as inactive in DB
//         state.db.update_item()
//             .table_name(&state.run_table)
//             .key("run_id", payload.run_id.clone().into())
//             .update_expression("SET is_active = :false")
//             .expression_attribute_values(":false", false.into())
//             .send()
//             .await?;
// 
//         // 4. CRITICAL: Cancel all pending alert-checker tasks for this run
//         if let Some((_, handles)) = state.active_tasks.remove(&payload.run_id) {
//             for handle in handles {
//                 handle.abort(); // Cancel the `tokio::time::sleep`
//             }
//             info!("Cancelled {} pending alert tasks for completed job run {}.",
//                 handles.len(), payload.run_id);
//         }
//     }
// 
//     Ok(StatusCode::OK)
// }
// 
// async fn is_stage_complete(
//     state: &AppState,
//     run_id: &str,
//     stage_name: &str,
// ) -> Result<bool, AppError> {
//     let run_item = state.db.get_item()
//         .table_name(&state.run_table)
//         .key("run_id", run_id.to_string().into())
//         .projection_expression("completed_stages")
//         .send()
//         .await?;
// 
//     match run_item.item() {
//         Some(item) => {
//             let run: JobRun = from_item(item.clone())?;
//             Ok(run.completed_stages.contains_key(stage_name))
//         }
//         None => {
//             // Job run not found, which is a problem
//             Err(AppError::NotFound(run_id.to_string()))
//         }
//     }
// }
// 
