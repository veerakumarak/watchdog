use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use crate::errors::AppError;

pub fn get_utc_now() -> DateTime<Utc> {
    Utc::now()
}
pub fn get_tz(zone: &String) -> Result<Tz, AppError> {
    let tz_res = zone.parse::<Tz>();
    match tz_res {
        Ok(tz) => Ok(tz),
        Err(_) => Err(AppError::BadRequest(format!("Invalid timezone provided: {}", zone)))
    }
}

// pub fn get_tz_now(zone: &String) -> Result<DateTime<Tz>, AppError> {
//     let utc_now = get_utc_now();
//     change_timezone(&utc_now, zone)
// }

pub fn change_timezone(dt: &DateTime<Utc>, zone: &String) -> Result<DateTime<Tz>, AppError> {
    let tz = get_tz(zone)?;
    change_tz(dt, &tz)
}

pub fn change_to_utc(dt: &DateTime<Tz>) -> Result<DateTime<Utc>, AppError> {
    Ok(dt.with_timezone(&Utc))
}

pub fn change_tz(dt: &DateTime<Utc>, tz: &Tz) -> Result<DateTime<Tz>, AppError> {
    Ok(dt.with_timezone(tz))
}
