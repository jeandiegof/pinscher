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
    fn start(&mut self) {
        self.start = Some(ProcessTime::now());
    }

    fn end(&mut self) {
        self.cpu_time = Some(self.start.unwrap().elapsed());
    }
}
