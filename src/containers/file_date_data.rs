use chrono::{DateTime, Utc};
use std::time::{Duration, SystemTime};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FileDateData {
    time: SystemTime,
}

impl FileDateData {
    pub fn new(time: SystemTime) -> Self {
        FileDateData { time }
    }
    pub fn format(&self) -> DateTime<Utc> {
        self.time.into()
    }
    pub fn time_since_zero(&self) -> Result<u64, String> {
        Ok(self
            .time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| "Time data apparently before Unix epoch")?
            .as_secs())
    }
}

pub fn secs_since_epoch_to_time(seconds: u64) -> SystemTime {
    return SystemTime::UNIX_EPOCH + Duration::from_secs(seconds);
}

#[derive(Debug, Error)]
pub enum DaysSinceNowToSystemTimeError {
    #[error("Failed to parse number to an unsigned intereger")]
    FailedToParseNumberToUInt,

    #[error("Number of days exceeds expected bounds, likely goes past unix epoch")]
    NumDaysExceedsExpectedBounds,
}

pub fn days_since_now_as_str_to_system_time(
    value: &str,
) -> Result<SystemTime, DaysSinceNowToSystemTimeError> {
    let num_days_as_seconds = value
        .parse::<u64>()
        .map_err(|_| DaysSinceNowToSystemTimeError::FailedToParseNumberToUInt)?
        * 86400;

    let days_since_now = SystemTime::now()
        .checked_sub(Duration::from_secs(num_days_as_seconds))
        .ok_or_else(|| ())
        .map_err(|_| DaysSinceNowToSystemTimeError::NumDaysExceedsExpectedBounds)?;

    Ok(days_since_now)
}
