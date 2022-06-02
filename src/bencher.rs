// a bencher trait? fn start, fn end
// the BenchSuite (currently named Bencher but that has to change)
// could store a object implementing the trait Bencher (ex: CpuTimeBencher, EnergyBencher, etc)
// and call start and stop of those methods.
// The open question is: how would the user get the output? Should the bench suite return it? should the user
// call .results() in the CpuTimeBencher (for example) and then receive a output struct specific for this type?
// That looks reasonable. Cons: initially I thought it would be hard to write the outputs of different benchers
// to a file, but we could implement Serializable for the custom benchs and that should work
// Another option: parametrize the type of the bencher. The bench function would become bench<F, A> where A is the type of the custom bencher.
// which has to implement the trait. The advantage is that we could return A as well in bench. So it would become bench<F, A>(function: F, bencher: A) -> A,
// where A: impl Bencher

// Does start and end really need mutable self?
// Maybe they could be start(self) -> Self instead
pub trait Bencher {
    fn start(self) -> Self;
    fn end(self) -> Self;
}

#[derive(Debug)]
pub struct BenchSuite {}

impl BenchSuite {
    pub fn bench<F, B>(function: F, bencher: B) -> B
    where
        F: Fn(),
        B: Bencher,
    {
        let started_bencher = bencher.start();
        function();
        started_bencher.end()
    }
}
