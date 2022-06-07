use crate::Error;

pub trait Bencher {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct BenchSuite {}

impl BenchSuite {
    pub fn bench<F, B>(function: F, bencher: &mut B) -> Result<(), Error>
    where
        F: Fn(),
        B: Bencher,
    {
        bencher.start()?;
        function();
        bencher.stop()?;

        Ok(())
    }
}
