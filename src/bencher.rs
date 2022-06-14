use crate::{Benchable, Error};

pub trait Bencher {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct BenchSuite;

impl BenchSuite {
    pub fn bench<A, B>(benchable: &mut A, bencher: &mut B) -> Result<(), Error>
    where
        A: Benchable,
        B: Bencher,
    {
        benchable.setup();

        bencher.start()?;
        benchable.execute();
        bencher.stop()?;

        benchable.teardown();

        Ok(())
    }
}
