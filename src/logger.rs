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

    print!("[{now}] >> {philo_id} ");

    match state {
        LogState::Death => println!("died"),
        LogState::Eating => println!("is eating"),
        LogState::Sleeping => println!("is sleeping"),
        LogState::Thinking => println!("is thinking"),
        LogState::NewFork => println!("has taken a fork"),
    }
}
