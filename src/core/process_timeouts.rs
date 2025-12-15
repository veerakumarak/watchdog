use tracing::{error, info};
use std::collections::{HashMap};
use std::ops::Sub;
use chrono::{Duration, DateTime, Utc};
use chrono_tz::Tz;
use chrono_tz::Tz::UTC;
use linked_hash_map::LinkedHashMap;
use crate::config::Config;
use crate::core::job_run_matching::detect_time_outs;
use crate::db::config_repository::get_all_enabled_configs;
use crate::db::connection::{DbConnection, PgPool};
use crate::db::run_repository::{get_all_pending_job_runs, insert_run, save_run};
use crate::errors::AppError;
use crate::models::{JobConfig, JobRun, JobRunStage, JobRunStatus, NewJobRun};
use crate::cron_utils::{get_job_start_time, in_between};
use crate::notification::core::{send_timeout};
use crate::notification::dispatcher::NotificationDispatcher;
use crate::time_utils::{change_to_utc, get_tz};

pub async fn check_all_timeouts(
    pool: &PgPool,
    notification_dispatcher: &NotificationDispatcher,
    config: &Config
) -> Result<(), AppError> {
    let mut conn = pool.get().await?;
    let all_enabled_configs: Vec<JobConfig> = get_all_enabled_configs(&mut conn).await?;
    if all_enabled_configs.is_empty() {
        return Ok(());
    }

    let jobs_by_name: HashMap<String, JobConfig> = all_enabled_configs.iter()
        .map(|job| (format!("{}-{}", &job.app_name, &job.job_name), job.clone()))
        .collect();

    let utc_now = Utc::now();
    // ToDo - add grace time to max_stage_duration_hours
    let time_boundary = utc_now.checked_sub_signed(Duration::hours(config.max_stage_duration_hours));
    let pending_events: Vec<JobRun> = get_all_pending_job_runs(&mut conn, time_boundary.unwrap()).await?;
    let latest_job_runs_by_name: HashMap<String, JobRun> = get_valid_events(&pending_events);

    let mut zoned_current_times: HashMap<Tz, DateTime<Tz>> = HashMap::default();
    zoned_current_times.insert(UTC, utc_now.with_timezone(&UTC));

    info!("utc time now: {}", utc_now);
    process_scheduled_job_timeouts(&mut conn, &all_enabled_configs, &latest_job_runs_by_name, &mut zoned_current_times, &utc_now, notification_dispatcher).await;
    process_manual_job_timeouts(&mut conn, pending_events, &jobs_by_name, &utc_now, notification_dispatcher).await;

    Ok(())
}

fn get_valid_events(pending_events: &Vec<JobRun>) -> HashMap<String, JobRun> {
    pending_events.into_iter().fold(HashMap::new(), |mut acc, job_run| {
        acc.entry(format!("{}-{}", job_run.app_name, job_run.job_name))
            .and_modify(|existing| {
                // Compare based on the logic: if existing is older, replace it
                if job_run.created_at > existing.created_at {
                    *existing = job_run.clone();
                }
            })
            .or_insert(job_run.clone());
        acc
    })
}

async fn process_scheduled_job_timeouts(
    conn: &mut DbConnection<'_>,
    all_enabled_jobs: &Vec<JobConfig>,
    latest_job_runs_by_name: &HashMap<String, JobRun>,
    zoned_current_times: &mut HashMap<Tz, DateTime<Tz>>,
    utc_now: &DateTime<Utc>,
    notification_dispatcher: &NotificationDispatcher,
) {
    for job_config in all_enabled_jobs.iter().filter(|job| job.schedule.is_some()) {
        if let Some(zone) = &job_config.zone_id && let Ok(tz) = get_tz(&zone) {
            let zoned_time_now = zoned_current_times
                .entry(tz)
                .or_insert_with(|| {utc_now.with_timezone(&tz)});
            match in_between(job_config, *zoned_time_now) {
                Err(e) => error!("Time delta calculation failed for {}: {:?}", job_config.job_name, e),
                Ok(true) => {
                    if let Ok(job_start_time) = get_job_start_time(job_config, zoned_time_now) {
                        let job_config_key = format!("{}-{}", job_config.app_name, job_config.job_name);
                        let job_run_option = latest_job_runs_by_name.get(&job_config_key).cloned();
                        // If event is null OR event.createdAt is before jobStartTime.minusMinutes(1)
                        let mut job_run: JobRun;
                        // ToDo add grace time
                        if job_run_option.is_none() || job_run_option.as_ref().unwrap().created_at < job_start_time.sub(Duration::seconds(10)) {
                            let new_job = NewJobRun {
                                app_name: job_config.app_name.clone(),
                                job_name: job_config.job_name.clone(),
                                status: JobRunStatus::InProgress,
                                stages: diesel_json::Json(Vec::new()),
                                triggered_at: utc_now.clone(),
                            };

                            if let Ok(_job_run) = insert_run(conn, new_job).await {
                                job_run = _job_run;
                            } else {
                                error!("Failed to insert job_run for job: {}", job_config.job_name);
                                continue;
                            }
                        } else {
                            job_run = job_run_option.unwrap();
                        }

                        update_event_stages(conn, job_config, zoned_time_now, &job_start_time, &mut job_run, notification_dispatcher).await;

                    } else {
                        // handle_failure(notification_dispatcher, job_config, &f).await;
                        error!("Failed to get job start time for job: {}", job_config.job_name);
                    }
                },
                _ => {}
            }
        }
    }
}

async fn process_manual_job_timeouts(
    conn: &mut DbConnection<'_>,
    pending_events: Vec<JobRun>,
    jobs_by_name: &HashMap<String, JobConfig>,
    utc_now: &DateTime<Utc>,
    notification_dispatcher: &NotificationDispatcher,
) {
    for mut job_run in pending_events.into_iter().filter(|e| e.status != JobRunStatus::Complete) {

        let job_config_key = format!("{}-{}", job_run.app_name, job_run.job_name);
        let job_config_option = jobs_by_name.get(&job_config_key);

        if let Some(job_config) = job_config_option.filter(|j| j.schedule.is_none()) {
            let job_start_time = job_run.triggered_at.with_timezone(&UTC);
            let tz_utc_now = utc_now.with_timezone(&UTC);

            update_event_stages(conn, job_config, &tz_utc_now, &job_start_time, &mut job_run, notification_dispatcher).await;
            //handle_failure(notification_dispatcher, job_config, &f).await;
        }
    }
}

async fn update_event_stages(
    conn: &mut DbConnection<'_>,
    job_config: &JobConfig,
    zoned_time_now: &DateTime<Tz>,
    job_start_time: &DateTime<Tz>,
    job_run: &mut JobRun,
    notification_dispatcher: &NotificationDispatcher,
) {
    let event_stages = detect_time_outs(job_config, job_run, zoned_time_now, job_start_time);

    error!("event_stages: {:?}", event_stages);

    if !event_stages.is_empty() {
        job_run.stages = diesel_json::Json(combine(&job_run.stages, event_stages));
        job_run.status = JobRunStatus::Failed;
        job_run.updated_at = change_to_utc(&zoned_time_now).unwrap();

        if let Err(e) = save_run(conn, job_run.clone()).await {
            error!("Failed to save job_run {}: {:?}", job_run.job_name, e);
            return;
        }

        for event_stage in job_run.stages.iter() {
            info!("in event timeout");
            send_timeout(&notification_dispatcher, &job_config, &job_run, &event_stage.name, &job_config.channel_ids).await
        }
    }
}

fn combine(stages1: &Vec<JobRunStage>, stages2: Vec<JobRunStage>) -> Vec<JobRunStage> {
    let mut combined_map = LinkedHashMap::new();

    for stage in stages1.iter() {
        combined_map.insert(stage.name.clone(), stage.clone());
    }

    // Iterate through the second list and overwrite entries with matching names
    for stage in stages2.into_iter() {
        combined_map.insert(stage.name.clone(), stage);
    }

    combined_map.values().cloned().collect()
}

// async fn handle_failure(
//     notification_dispatcher: &NotificationDispatcher,
//     job_config: &JobConfig,
//     f: &AppError
// ) {
//     error!("Error checking timeout for job: {}. Reason: {}", &job_config.job_name, f);
//     send_error(&notification_dispatcher, &job_config.application, &job_config.job_name, f.to_string().as_ref(), vec!["slack_webhook".to_string()]).await;
// }
