pub enum Error {
    FailedToAcquireEnergyData(powercap::ReadError),
    FailedToUsePowercapFramework(powercap::BuildError),
    CoreEnergyNotAvailable,
    PackageEnergyNotAvailable,
    TimeBencherNotStarted,
    TimeBencherNotStopped,
}

impl From<powercap::BuildError> for Error {
    fn from(error: powercap::BuildError) -> Self {
        Error::FailedToUsePowercapFramework(error)
    }
}

impl From<powercap::ReadError> for Error {
    fn from(error: powercap::ReadError) -> Self {
        Error::FailedToAcquireEnergyData(error)
    }
}
