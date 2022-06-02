use cpu_time::ProcessTime;
use std::time::Duration;

#[derive(Debug)]
pub struct Bencher {}

impl Bencher {
    pub fn bench<F>(function: F) -> BenchResult
    where
        F: Fn(),
    {
        let start = ProcessTime::now();
        function();

        BenchResult {
            cpu_time: start.elapsed(),
        }
    }
}

#[derive(Debug)]
pub struct BenchResult {
    cpu_time: Duration,
}

impl BenchResult {
    pub fn cpu_time(&self) -> Duration {
        self.cpu_time
    }
}
