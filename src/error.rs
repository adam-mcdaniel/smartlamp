use govee_rs::error::GoveeError;
use std::fmt;

pub enum Error {
    GoveeError(GoveeError),
    NoDevicesFound,
    NoMatchingDevices,
}

impl From<GoveeError> for Error {
    fn from(e: GoveeError) -> Self {
        Self::GoveeError(e)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::GoveeError(e) => write!(f, "{:?}", e),
            Self::NoDevicesFound => write!(f, "No devices found"),
            Self::NoMatchingDevices => write!(f, "No matching devices"),
        }
    }
}

