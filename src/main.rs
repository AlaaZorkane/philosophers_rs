#![allow(dead_code, unused_variables)]
pub mod config;
pub mod fork;
pub mod logger;
pub mod philo;
pub mod utils;

use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    config::Config,
    fork::Fork,
    logger::{log_state, LogState},
    philo::Philo,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_len = args.len();
    if args_len < 5 {
        // TODO: use custom errors with display yada yada
        eprintln!("Invalid args count, expected 4+ got {args_len}.");
        println!("usage: philo <number_of_philosophers> <time_to_die> <time_to_eat> <time_to_sleep> [number_of_times_each_philosopher_must_eat]");
        return;
    };

    let config = Arc::new(Config {
        n_philo: args[1].parse().expect("Invalid philosophers count"),
        t_die: Duration::from_millis(args[2].parse().expect("Invalid death time")),
        t_eat: Duration::from_millis(args[3].parse().expect("Invalid eat time")),
        t_sleep: Duration::from_millis(args[4].parse().expect("Invalid sleep time")),
        n_eat: if let Some(val) = args.get(5) {
            val.parse().ok()
        } else {
            None
        },
    });

    println!("Config: {:#?}", config);

    let mut handles = vec![];
    let v: i32 = (1..5).collect::<Vec<i32>>().iter().map(|x| x.pow(2)).sum();
    let forks: Vec<_> = (0..config.n_philo as u32)
        .into_iter()
        .map(|fork_id| Arc::new(Mutex::new(Fork::new(fork_id))))
        .collect();

    for philo_id in 1..=(config.n_philo as u32) {
        let config_clone = config.clone();

        // In a circular way (L,R)
        let fork_l_index = (philo_id - 1) as usize;
        let fork_r_index = if philo_id == 1 {
            config.n_philo - 1
        } else {
            (philo_id - 2) as usize
        };

        let (fork_l, fork_r) = (
            forks.get(fork_l_index).unwrap().clone(),
            forks.get(fork_r_index).unwrap().clone(),
        );

        let handle = thread::spawn(move || {
            let mut philo = Philo::new(philo_id, config_clone, fork_l, fork_r);

            loop {
                if let Err(_) = philo.eat() {
                    break;
                }
                if let Err(_) = philo.sleep() {
                    break;
                }
                if let Err(_) = philo.think() {
                    break;
                }
            }

            log_state(philo_id, LogState::Death);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
