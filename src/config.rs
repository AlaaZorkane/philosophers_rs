use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// `number_of_philosophers` is the number of philosophers and also the number of forks.
    pub n_philo: usize,
    /// `time_to_die` is in milliseconds, if a philosopher doesn’t start eating `time_to_die`
    /// milliseconds after starting their last meal or the beginning of the simulation,
    /// it dies.
    pub t_die: Duration,
    /// `time_to_eat` is in milliseconds and is the time it takes for a philosopher to
    /// eat. During that time they will need to keep the two forks.
    pub t_eat: Duration,
    /// `time_to_sleep` is in milliseconds and is the time the philosopher will spend
    /// sleeping.
    pub t_sleep: Duration,
    /// `number_of_times_each_philosopher_must_eat` argument is optional, if all
    /// philosophers eat at least `number_of_times_each_philosopher_must_eat` the
    /// simulation will stop. If not specified, the simulation will stop only at the death
    /// of a philosopher.
    pub n_eat: Option<usize>,
}
