use std::collections::HashMap;
use std::ops::Add;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use tracing::{debug, warn};
use crate::cron_utils::get_min;
use crate::models::{JobConfig, JobRun, JobRunStage, JobRunStageStatus, JobRunStatus};
use crate::models::JobRunStageStatus::Missed;

pub fn detect_time_outs(
    job_config: &JobConfig,
    latest_job_run: &JobRun,
    current_time: &DateTime<Tz>,
    job_start_time: &DateTime<Tz>,
) -> Vec<JobRunStage> {
    let occurring_stages_map = get_event_stage_map(latest_job_run);

    // We need an owned version of the stages to sort them.
    let mut sorted_job_stages = job_config.stages.to_vec();

    // Java used Comparator.comparingInt(JobStageUtils::getMin)
    // Assuming this sorts by the 'start' offset, putting None last (or first depending on requirement).
    // Here we assume older start times come first.
    sorted_job_stages.sort_by_key(|stage| get_min(stage.start.clone(), stage.complete.clone()).unwrap());

    sorted_job_stages
        .into_iter()
        .filter_map(|job_stage| {
            // Get an owned copy of the existing stage, or create a new brand new one
            let mut occurring_stage = occurring_stages_map
                .get(&job_stage.name)
                .cloned()
                .unwrap_or_else(|| JobRunStage{
                    name: job_stage.name.clone(),
                    start_status: None,
                    start_date_time: None,
                    complete_status: None,
                    complete_date_time: None,
                });

            let mut updated = false;

            // Check Start Timeout
            if let Some(start_offset_secs) = job_stage.start {
                if occurring_stage.start_status.is_none() {
                    // chrono::Duration is used for time arithmetic
                    let deadline = job_start_time.add(Duration::seconds(start_offset_secs as i64));
                    if deadline < *current_time {
                        debug!("Detected start timeout for stage: {}", job_stage.name);
                        occurring_stage.start_status = Some(Missed);
                        occurring_stage.start_date_time = Some(current_time.with_timezone(&Utc));
                        updated = true;
                    }
                }
            }

            // Check Complete Timeout
            if let Some(complete_offset_secs) = job_stage.complete {
                if occurring_stage.complete_status.is_none() {
                    let deadline = job_start_time.add(Duration::seconds(complete_offset_secs as i64));
                    if deadline < *current_time {
                        debug!("Detected complete timeout for stage: {}", job_stage.name);
                        occurring_stage.complete_status = Some(Missed);
                        occurring_stage.complete_date_time = Some(current_time.with_timezone(&Utc));
                        updated = true;
                    }
                }
            }

            if updated { Some(occurring_stage) } else { None }
        })
        .collect()
}

pub fn get_event_stage_map(job_run: &JobRun) -> HashMap<String, JobRunStage> {
    let mut map = HashMap::new();
    for stage in job_run.stages.iter() {
        map.entry(stage.name.clone())
            .or_insert_with(|| stage.clone());
    }
    map
}

pub fn get_status(job_config: &JobConfig, job_run: &JobRun) -> JobRunStatus {
    if job_run.status != JobRunStatus::InProgress {
        return job_run.status.clone();
    }

    let occurring_stages_map = get_event_stage_map(job_run);

    let mut is_missed = false;
    for job_stage in job_config.stages.iter() {
        // If the map doesn't contain the key
        if !occurring_stages_map.contains_key(&job_stage.name) {
            is_missed = true;
            continue;
        }

        // We know it exists now, unwrap is safe
        let occurring_stage = occurring_stages_map.get(&job_stage.name).unwrap();

        // Check Start Status
        // If job config requires start AND event has a start status recorded
        if job_stage.start.is_some() && occurring_stage.start_status.is_some() {
            // If that recorded status is not Occurred (e.g., it's Missed or Failed)
            if *occurring_stage.start_status.as_ref().unwrap() != JobRunStageStatus::Occurred {
                warn!("Job failed due to non-occurred start status in stage: {}", job_stage.name);
                return JobRunStatus::Failed;
            }
        }

        // Check Complete Status
        // If job config requires complete AND event has a complete status recorded
        if job_stage.complete.is_some() && occurring_stage.complete_status.is_some() {
            // If that recorded status is not Occurred
            if *occurring_stage.complete_status.as_ref().unwrap() != JobRunStageStatus::Occurred {
                warn!("Job failed due to non-occurred complete status in stage: {}", job_stage.name);
                return JobRunStatus::Failed;
            }
        }
    }

    if is_missed {
        JobRunStatus::InProgress
    } else {
        JobRunStatus::Complete
    }
}
