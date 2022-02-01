use std::sync::Weak;

use crate::philo::Philo;

type HolderLink = Weak<Philo>;

pub struct Fork {
    /// TODO: Who's currently holding the fork
    ///
    /// NOTE: Don't forget to implement a Drop for it
    holder: Option<HolderLink>,
    // from 0 to `n_philo` + 1
    id: u32,
}

impl Fork {
    pub fn new(id: u32) -> Self {
        Self { id, holder: None }
    }

    pub fn pick(&mut self, new_holder: HolderLink) {
        self.holder = Some(new_holder);
    }
}
