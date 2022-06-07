#[derive(Debug)]
pub enum Error {
    FailedToAcquireEnergyData(powercap::ReadError),
    FailedToUsePowercapFramework(powercap::BuildError),
    CoreEnergyNotAvailable,
    PackageEnergyNotAvailable,
    BencherNotStarted,
    BencherNotStopped,
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
