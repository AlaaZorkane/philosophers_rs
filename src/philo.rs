use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    fork::Fork,
    logger::{log_state, LogState},
    utils::now,
    Config,
};

enum PhiloState {
    Eating,
    Sleeping,
    Thinking,
}

type ForkLink = Arc<Mutex<Fork>>;

pub struct Philo {
    state: PhiloState,
    /// sequential number of philosopher (starting 1)
    id: u32,
    /// Yes we could technically just pass the times from the main
    /// thread to the functions but meh
    config: Arc<Config>,
    /// The fork on the left of the philosopher
    fork_l: ForkLink,
    /// The fork on the right of the philosopher
    fork_r: ForkLink,
    /// When was the last time this philosopher has eaten
    last_eat: Duration,
    /// When did the philosopher die
    death_time: Option<Duration>,
}

impl Philo {
    pub fn new(id: u32, config: Arc<Config>, fork_l: ForkLink, fork_r: ForkLink) -> Self {
        Self {
            id,
            state: PhiloState::Thinking,
            config,
            fork_l,
            fork_r,
            last_eat: now(),
            death_time: None,
        }
    }

    pub fn eat(&mut self) -> Result<(), ()> {
        let fork_l = self.fork_l.lock().unwrap();
        if self.is_dead() {
            return Err(());
        };

        let fork_r = self.fork_r.lock().unwrap();

        self.last_eat = now();
        self.state = PhiloState::Eating;
        // fork_l.holder = Some(self);
        log_state(self.id, LogState::Eating);

        thread::sleep(self.config.t_eat);

        Ok(())
    }

    pub fn sleep(&mut self) -> Result<(), ()> {
        if self.is_dead() {
            return Err(());
        };

        self.state = PhiloState::Sleeping;
        log_state(self.id, LogState::Sleeping);

        thread::sleep(self.config.t_sleep);

        Ok(())
    }

    pub fn think(&mut self) -> Result<(), ()> {
        self.state = PhiloState::Thinking;
        log_state(self.id, LogState::Thinking);

        // thread::sleep(self.config.t_die);
        Ok(())
    }

    pub fn is_dead(&self) -> bool {
        match self.death_time {
            None => {
                let t_die = self.config.t_die;
                let last_eat = self.last_eat;
                let now = now();

                let is_dead = now > last_eat + t_die;
                if is_dead {
                    // TODO: Figure out a way to mutate here
                    // self.death_time = Some(now);
                }

                is_dead
            }
            Some(_) => true,
        }
    }
}
