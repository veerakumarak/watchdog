use std::ops::{Add, Sub};
use crate::models::JobConfig;
use chrono::{DateTime, Duration, TimeZone};
use chrono_tz::Tz;
use cron::Schedule;
use crate::errors::AppError;
use std::str::FromStr;

pub fn get_min(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        _ => None,
    }
}

pub fn get_max(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        _ => None,
    }
}

pub fn get_cron_start_time(job: &JobConfig, current_time: &DateTime<Tz>) -> Result<DateTime<Tz>, AppError> {
    // let min_duration: i64 = job.stages.iter().filter_map(|a| get_min(a.start, a.complete)).min().unwrap_or_else(|| 0) as i64;
    // let reference_start_time = current_time.sub(Duration::seconds(min_duration));

    if let Some(cron) = &job.schedule {
        get_previous_execution_time(&cron, &current_time)
    } else {
        Err(AppError::InternalError("schedule is expected".parse().unwrap()))
    }
}

pub fn get_job_start_time(job: &JobConfig, current_time: &DateTime<Tz>) -> Result<DateTime<Tz>, AppError> {
    get_cron_start_time(job, current_time)
}

pub fn get_job_complete_time(job: &JobConfig, current_time: DateTime<Tz>) -> Result<DateTime<Tz>, AppError> {
    let max_duration = job
        .stages
        .iter()
        .map(|a| {
            get_max(a.start, a.complete).unwrap_or_else(|| i64::MAX as u64)
            // if a.complete.is_some() {
            //     a.complete.unwrap()
            // } else {
            //     a.start.unwrap()
            // }
        })
        .max()
        .unwrap_or_else(|| i64::MAX as u64);

    let start_time = get_cron_start_time(job, &current_time)?;
    Ok(start_time.add(Duration::seconds(max_duration as i64)))
}

pub fn in_between(job: &JobConfig, current_time: DateTime<Tz>) -> Result<bool, AppError> {
    let job_start_time = get_job_start_time(job, &current_time)?;
    let job_complete_time = get_job_complete_time(job, current_time)?;

    let buffer_complete_time = job_complete_time.add(Duration::minutes(2));

    let is_after_start = current_time > job_start_time;
    let is_before_complete = current_time < buffer_complete_time;

    Ok(is_after_start && is_before_complete)
}

pub fn get_previous_execution_time<TZ>(
    cron_string: &str,
    from_date_time: &DateTime<TZ>,
) -> Result<DateTime<TZ>, AppError> where
    TZ: TimeZone + Clone,
    <TZ as TimeZone>::Offset: Copy, // Required for proper trait resolution
{
    let schedule_result = Schedule::from_str(cron_string);
    if schedule_result.is_err() {
        return Err(AppError::InternalError(format!("invalid schedule found: {}", cron_string)))
    }
    let schedule = schedule_result.unwrap();

    // let from_utc: DateTime<Utc> = from_date_time.with_timezone(&Utc);

    // 2. Find the last execution time (executionTime.lastExecution(fromDateTime))
    // We get an iterator that yields a maximum of one previous execution.
    // The `cron` crate's `upcoming_owned()` or `after_owned()` are common,
    // but we use `before_owned()` for previous/last execution.
    // let last_execution_iter = schedule.before_owned(from_utc);
    // let last_execution: Option<DateTime<Utc>> = last_execution_iter.next();
    //
    let last_execution: Option<DateTime<TZ>> = schedule
        .after(&from_date_time)
        .rev() // Reverse the iterator to get past times
        .next(); // Take the first one (the most recent past time)

    // 3. Handle the result and error (if lastExecution.isEmpty() throw new InternalException)
    match last_execution {
        Some(dt_utc) => {
            // Convert the result back to the original time zone (ZonedDateTime)
            let result_zdt = dt_utc.with_timezone(&from_date_time.timezone());
            Ok(result_zdt)
        }
        None => Err(AppError::InternalError(cron_string.to_string())),
    }
}
