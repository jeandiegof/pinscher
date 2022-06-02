use powercap::PowerCap;

fn energy_begin() {
    let intel_rapl = PowerCap::try_default().unwrap().intel_rapl;

    // this is supposed to be intel-rapl/intel-rapl:0, which is "package"
    // cat /sys/class/powercap/intel-rapl/intel-rapl:0/name
    let package = intel_rapl.sockets.get(&0).unwrap();
    let package_energy = package.energy().unwrap();

    // this is supposed to be /sys/class/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:0
    // cat /sys/class/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:0/name
    let core = package.domains.get(&0).unwrap();
    let core_energy = core.energy().unwrap();

    // println!("{:#?}", powercap.intel_rapl.snapshot());
    println!("package: {:?} core: {:?}", package_energy, core_energy);
    assert!(package_energy > core_energy);

    // TODO: which values are we interested in? How could we generalize it for other processors?
    // Mine has only two the package and the core, but others may have more sensors.
    // How to customize the bench so that we can use custom benchers that return different types of BenchResult?
    // If we have a CpuTimeBencher and a EnergyBencher, we could have CpuTimeBenchResult and EnergyBenchResult, but that
    // wouldn't scale well.
    // At the end, what matters for us to be able to serialize a BenchResult. If that's true, Bencher::bench could return
    // something that implements a serializable trait.
}

fn energy_end() {
    let powercap = PowerCap::try_default().unwrap();
    println!("Total energy: {:?} uJ", powercap.intel_rapl.total_energy());
    // println!("{:#?}", powercap);
}

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
