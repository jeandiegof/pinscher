use crate::{Bencher, Error};
use std::time::{Duration, Instant};

pub struct TimeBencher {
    start: Option<Instant>,
    time: Option<Duration>,
}

impl TimeBencher {
    pub fn new() -> Self {
        Self {
            start: None,
            time: None,
        }
    }

    /// Returns the time taken by the function being benchmarked.
    pub fn real_time(&self) -> Result<Duration, Error> {
        self.time.ok_or(Error::BencherNotStopped)
    }
}

impl Default for TimeBencher {
    fn default() -> Self {
        Self::new()
    }
}

impl Bencher for TimeBencher {
    fn start(&mut self) -> Result<(), Error> {
        self.start = Some(Instant::now());

        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        self.time = Some(self.start.ok_or(Error::BencherNotStarted)?.elapsed());

        Ok(())
    }
}
