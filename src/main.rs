#![allow(dead_code, unused_variables)]
pub mod config;
pub mod logger;
pub mod philo;

use std::{env, sync::Arc, thread, time::Duration};

use crate::{
    config::Config,
    philo::{Fork, Philo},
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

    // let mut philo_vec = Vec::<Philo>::new();
    let mut handles = vec![];
    let forks: Vec<Fork> = vec![0; config.n_philo]
        .into_iter()
        .map(|fork_id| Fork::new(fork_id))
        .collect();

    for philo_id in 1..(config.n_philo as u32) {
        let config_clone = Arc::clone(&config);

        let fork = Fork::new(philo_id);
        let handle = thread::spawn(move || {
            let philo = Philo::new(philo_id, config_clone);

            loop {
                philo.eat();
                philo.sleep();
                philo.think();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
