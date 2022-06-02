use cpu_time::ProcessTime;
use std::time::Duration;

pub struct Bencher {}

impl Bencher {
    pub fn bench<F>(function: F) -> Duration
    where
        F: Fn(),
    {
        let start = ProcessTime::now();
        function();
        start.elapsed()
    }
}
