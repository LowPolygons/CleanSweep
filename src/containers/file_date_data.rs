use chrono::{DateTime, Utc};
use std::time::{Duration, SystemTime};

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
    pub fn get_time(&self) -> &SystemTime {
        &self.time
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
