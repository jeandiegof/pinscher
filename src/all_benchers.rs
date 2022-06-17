use crate::{Bencher, CpuTimeBencher, EnergyBencher, Error, TimeBencher};

pub struct AllBenchers {
    cpu_time_bencher: CpuTimeBencher,
    energy_bencher: EnergyBencher,
    time_bencher: TimeBencher,
}

impl AllBenchers {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            cpu_time_bencher: CpuTimeBencher::new(),
            energy_bencher: EnergyBencher::new()?,
            time_bencher: TimeBencher::new(),
        })
    }

    pub fn cpu_time_bencher(&self) -> &CpuTimeBencher {
        &self.cpu_time_bencher
    }

    pub fn energy_bencher(&self) -> &EnergyBencher {
        &self.energy_bencher
    }

    pub fn time_bencher(&self) -> &TimeBencher {
        &self.time_bencher
    }
}

impl Bencher for AllBenchers {
    fn start(&mut self) -> Result<(), Error> {
        self.energy_bencher.start()?;
        self.time_bencher.start()?;
        self.cpu_time_bencher.start()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        self.energy_bencher.stop()?;
        self.time_bencher.stop()?;
        self.cpu_time_bencher.stop()?;

        Ok(())
    }
}
