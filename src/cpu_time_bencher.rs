use crate::bencher::Bencher;
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

    // Returns the active cpu_time used. Panics if called before Bencher::start is called.
    pub fn cpu_time(&self) -> Duration {
        self.cpu_time.unwrap()
    }
}

impl Bencher for CpuTimeBencher {
    fn start(self) -> Self {
        Self {
            start: Some(ProcessTime::now()),
            ..self
        }
    }

    fn end(self) -> Self {
        Self {
            cpu_time: Some(self.start.unwrap().elapsed()),
            ..self
        }
    }
}
