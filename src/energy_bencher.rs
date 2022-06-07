use crate::bencher::Bencher;
use powercap::{DomainSnapshot, IntelRapl, IntelRaplSnapshot, PowerCap, SocketSnapshot};

pub struct EnergyBencher {
    intel_rapl: IntelRapl,
    start_energy_snapshot: Option<IntelRaplSnapshot>,
    end_energy_snapshot: Option<IntelRaplSnapshot>,
    package_energy: u64,
    core_energy: u64,
}

// TODO: overflow handling?? there's socket max range
// TODO: options for core energy and package energy?
// TODO: error handling (implies changing the signature of the crate)
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
        let max = snapshot_end
            .max_energy_range
            .max(snapshot_begin.max_energy_range);

        self.package_energy = Self::overflow_sub(snapshot_end.energy, snapshot_begin.energy, max);
    }

    fn calculate_core_energy_consumption(&mut self) {
        let snapshot_begin = Self::core_snapshot(self.start_energy_snapshot.as_ref().unwrap());
        let snapshot_end = Self::core_snapshot(self.end_energy_snapshot.as_ref().unwrap());
        let max = snapshot_end
            .max_energy_range
            .max(snapshot_begin.max_energy_range);

        self.core_energy = Self::overflow_sub(snapshot_end.energy, snapshot_begin.energy, max);
    }

    /// Calculates a - b considering that the two values vary in a range [0, max_value]
    fn overflow_sub(a: u64, b: u64, max_value: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            max_value - (b - a)
        }
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
    fn start(&mut self) {
        self.start_energy_snapshot = self.intel_rapl.snapshot().ok();
    }

    fn end(&mut self) {
        self.end_energy_snapshot = self.intel_rapl.snapshot().ok();
        self.calculate_energy_consumption();
    }
}
