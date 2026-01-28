use chrono::{DateTime, Utc};
use std::time::SystemTime;

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
}
