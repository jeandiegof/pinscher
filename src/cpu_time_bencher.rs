use crate::{Bencher, Error};
use cpu_time::ProcessTime;
use std::time::Duration;

pub struct CpuTimeBencher {
    start: Option<ProcessTime>,
    cpu_time: Option<Duration>,
}

impl CpuTimeBencher {
    pub fn new() -> Self {
        Self {
            start: None,
            cpu_time: None,
        }
    }

    /// Returns the active CPU time consumed by the function being benchmarked.
    pub fn cpu_time(&self) -> Result<Duration, Error> {
        self.cpu_time.ok_or(Error::BencherNotStopped)
    }
}

impl Default for CpuTimeBencher {
    fn default() -> Self {
        Self::new()
    }
}

impl Bencher for CpuTimeBencher {
    fn start(&mut self) -> Result<(), Error> {
        self.start = Some(ProcessTime::now());

        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        self.cpu_time = Some(self.start.ok_or(Error::BencherNotStarted)?.elapsed());

        Ok(())
    }
}
