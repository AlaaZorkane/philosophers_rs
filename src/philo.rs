use std::{sync::Arc, thread, time::Duration};

use crate::{
    logger::{log_state, LogState},
    Config,
};

enum PhiloState {
    Eating,
    Sleeping,
    Thinking,
}

pub struct Philo {
    state: PhiloState,
    /// sequential number of philosopher (starting 1)
    id: u32,
    /// Yes we could technically just pass the times from the main
    /// thread to the functions but meh
    config: Arc<Config>,
}

impl Philo {
    pub fn new(id: u32, config: Arc<Config>) -> Philo {
        Philo {
            id,
            state: PhiloState::Thinking,
            config,
        }
    }

    pub fn eat(&self) {
        log_state(self.id, LogState::Eating);

        thread::sleep(self.config.t_eat);
    }

    pub fn sleep(&self) {
        log_state(self.id, LogState::Sleeping);

        thread::sleep(self.config.t_sleep);
    }

    pub fn think(&self) {
        log_state(self.id, LogState::Thinking)
    }
}

pub struct Fork<'philo> {
    /// Who's currently holding the fork
    holder: Option<&'philo Philo>,
    id: u32,
}

impl<'philo> Fork<'philo> {
    pub fn new(id: u32) -> Fork<'philo> {
        Fork { id, holder: None }
    }

    pub fn pick(&mut self, new_holder: &'philo Philo) {
        self.holder = Some(new_holder);
    }
}
