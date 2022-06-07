pub trait Bencher {
    fn start(&mut self);
    fn end(&mut self);
}

#[derive(Debug)]
pub struct BenchSuite {}

impl BenchSuite {
    pub fn bench<F, B>(function: F, bencher: &mut B)
    where
        F: Fn(),
        B: Bencher,
    {
        bencher.start();
        function();
        bencher.end();
    }
}
