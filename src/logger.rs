use std::time::{SystemTime, UNIX_EPOCH};

pub enum LogState {
    Death,
    Eating,
    Sleeping,
    Thinking,
    NewFork,
}

/// Logs current philosopher state as `timestamp_in_ms X has taken a fork`
pub fn log_state(philo_id: u32, state: LogState) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut msg = String::from(format!("[{now}] >> {philo_id} "));
    match state {
        LogState::Death => msg.push_str("died :("),
        LogState::Eating => msg.push_str("is eating"),
        LogState::Sleeping => msg.push_str("is sleeping"),
        LogState::Thinking => msg.push_str("is thinking"),
        LogState::NewFork => msg.push_str("has taken a fork"),
    };

    println!("{msg}");
}
