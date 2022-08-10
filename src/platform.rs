use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum Platform {
    OSX,
    Linux(LinuxDistro),
    Windows,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LinuxDistro {
    Ubuntu,
    Debian,
}

#[derive(Debug)]
pub struct UnknownPlatformError;

impl FromStr for Platform {
    type Err = UnknownPlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OSX" => Ok(Self::OSX),
            "Windows" => Ok(Self::Windows),
            "Ubuntu" => Ok(Self::Linux(LinuxDistro::Ubuntu)),
            "Debian" => Ok(Self::Linux(LinuxDistro::Debian)),
            _ => Err(UnknownPlatformError),
        }
    }
}
