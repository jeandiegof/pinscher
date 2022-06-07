use crate::{Bencher, Error};
use powercap::{DomainSnapshot, IntelRapl, IntelRaplSnapshot, PowerCap, SocketSnapshot};

pub struct EnergyBencher {
    intel_rapl: IntelRapl,
    first_snapshot: Option<IntelRaplSnapshot>,
    final_snapshot: Option<IntelRaplSnapshot>,
    package_energy: u64,
    core_energy: u64,
}

impl EnergyBencher {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            intel_rapl: PowerCap::try_default()?.intel_rapl,
            first_snapshot: None,
            final_snapshot: None,
            package_energy: 0,
            core_energy: 0,
        })
    }

    /// Returns the energy consumed by the package between start and end in micro Joules
    pub fn package_energy(&self) -> u64 {
        self.package_energy
    }

    /// Returns the energy consumed by the core between start and end in micro Joules
    pub fn core_energy(&self) -> u64 {
        self.core_energy
    }

    fn calculate_energy_consumption(&mut self) -> Result<(), Error> {
        self.calculate_package_energy_consumption()?;
        self.calculate_core_energy_consumption()?;

        Ok(())
    }

    fn calculate_package_energy_consumption(&mut self) -> Result<(), Error> {
        let snapshot_begin = Self::package_snapshot(self.first_snapshot.as_ref().unwrap())?;
        let snapshot_end = Self::package_snapshot(self.final_snapshot.as_ref().unwrap())?;
        let max = snapshot_end.max_energy_range;

        self.package_energy = Self::overflow_sub(snapshot_end.energy, snapshot_begin.energy, max);

        Ok(())
    }

    fn calculate_core_energy_consumption(&mut self) -> Result<(), Error> {
        let snapshot_begin = Self::core_snapshot(self.first_snapshot.as_ref().unwrap())?;
        let snapshot_end = Self::core_snapshot(self.final_snapshot.as_ref().unwrap())?;
        let max = snapshot_end.max_energy_range;

        self.core_energy = Self::overflow_sub(snapshot_end.energy, snapshot_begin.energy, max);

        Ok(())
    }

    /// Calculates a - b considering that the two values vary in a range [0, max_value]
    fn overflow_sub(a: u64, b: u64, max_value: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            max_value - (b - a)
        }
    }

    fn package_snapshot(snapshot: &IntelRaplSnapshot) -> Result<&SocketSnapshot, Error> {
        snapshot
            .sockets
            .iter()
            .find(|&s| s.id == 0)
            .ok_or(Error::PackageEnergyNotAvailable)
    }

    fn core_snapshot(snapshot: &IntelRaplSnapshot) -> Result<&DomainSnapshot, Error> {
        Self::package_snapshot(snapshot)?
            .domains
            .iter()
            .find(|&d| d.id == 0)
            .ok_or(Error::CoreEnergyNotAvailable)
    }
}

impl Bencher for EnergyBencher {
    fn start(&mut self) -> Result<(), Error> {
        self.first_snapshot = Some(self.intel_rapl.snapshot()?);

        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        self.final_snapshot = Some(self.intel_rapl.snapshot()?);
        self.calculate_energy_consumption()?;

        Ok(())
    }
}
