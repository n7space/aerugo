//! System scheduler.

use crate::aerugo::Aerugo;
use crate::api::RuntimeApi;
use crate::arch::logln;

/// System scheduler.
pub(crate) struct Executor {
    system_api: &'static Aerugo,
}

impl Executor {
    pub(crate) const fn new(system_api: &'static Aerugo) -> Self {
        Executor { system_api }
    }

    /// Starts tasklet scheduling.
    pub(crate) fn run_scheduler(&self) -> ! {
        loop {
            logln!("{}", self.system_api.get_system_time())
        }
    }
}
