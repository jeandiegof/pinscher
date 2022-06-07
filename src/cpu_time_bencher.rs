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

    /// Returns the active cpu_time used.
    pub fn cpu_time(&self) -> Duration {
        self.cpu_time.unwrap()
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

    fn end(&mut self) -> Result<(), Error> {
        // TODO: return an error instead of unwrapping
        self.cpu_time = Some(self.start.unwrap().elapsed());

        Ok(())
    }
}
