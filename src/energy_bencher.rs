use crate::bencher::Bencher;
use powercap::{DomainSnapshot, IntelRapl, IntelRaplSnapshot, SocketSnapshot};

pub struct EnergyBencher {
    intel_rapl: IntelRapl,
    start_energy_snapshot: Option<IntelRaplSnapshot>,
    end_energy_snapshot: Option<IntelRaplSnapshot>,
    package_energy: u64,
    core_energy: u64,
}

// TODO: overflow handling?? there's socket max range
// TODO: options for core energy and package energy?
impl EnergyBencher {
    pub fn new() -> Self {
        Self {
            intel_rapl: PowerCap::try_default().unwrap().intel_rapl,
            start_energy_snapshot: None,
            end_energy_snapshot: None,
            package_energy: 0,
            core_energy: 0,
        }
    }

    /// Returns the energy consumed by the package between start and end
    pub fn package_energy(&self) -> u64 {
        self.package_energy
    }

    /// Returns the energy consumed by the core between start and end
    pub fn core_energy(&self) -> u64 {
        self.core_energy
    }

    fn calculate_energy_consumption(&mut self) {
        self.calculate_package_energy_consumption();
        self.calculate_core_energy_consumption();
    }

    fn calculate_package_energy_consumption(&mut self) {
        let snapshot_begin = Self::package_snapshot(self.start_energy_snapshot.as_ref().unwrap());
        let snapshot_end = Self::package_snapshot(self.end_energy_snapshot.as_ref().unwrap());

        self.package_energy = snapshot_end.energy - snapshot_begin.energy;
    }

    fn calculate_core_energy_consumption(&mut self) {
        let snapshot_begin = Self::core_snapshot(self.start_energy_snapshot.as_ref().unwrap());
        let snapshot_end = Self::core_snapshot(self.end_energy_snapshot.as_ref().unwrap());

        self.core_energy = snapshot_end.energy - snapshot_begin.energy;
    }

    fn package_snapshot(snapshot: &IntelRaplSnapshot) -> &SocketSnapshot {
        snapshot.sockets.iter().find(|&s| s.id == 0).unwrap()
    }

    fn core_snapshot(snapshot: &IntelRaplSnapshot) -> &DomainSnapshot {
        Self::package_snapshot(snapshot)
            .domains
            .iter()
            .find(|&d| d.id == 0)
            .unwrap()
    }
}

impl Bencher for EnergyBencher {
    // TODO: change the signature of the trait to return a result
    fn start(&mut self) {
        self.start_energy_snapshot = self.intel_rapl.snapshot().ok();
    }

    fn end(&mut self) {
        self.end_energy_snapshot = self.intel_rapl.snapshot().ok();
        self.calculate_energy_consumption();
    }
}

use powercap::PowerCap;

fn energy_begin() {
    let intel_rapl = PowerCap::try_default().unwrap().intel_rapl;

    // this is supposed to be intel-rapl/intel-rapl:0, which is "package"
    // cat /sys/class/powercap/intel-rapl/intel-rapl:0/name
    let package = intel_rapl.sockets.get(&0).unwrap();
    let package_energy = package.energy().unwrap();

    // this is supposed to be intel-rapl/intel-rapl:0/intel-rapl:0:0, which is "core"
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
