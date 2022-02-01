use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}
