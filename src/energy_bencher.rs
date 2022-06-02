use crate::bencher::Bencher;

pub struct EnergyBencher {}

impl EnergyBencher {
    pub fn new() -> Self {
        unimplemented!()
    }
}

impl Bencher for EnergyBencher {
    fn start(&mut self) {
        unimplemented!();
    }

    fn end(&mut self) {
        unimplemented!();
    }
}

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
