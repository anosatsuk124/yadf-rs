use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum PackageManager {
    Apt,
    Brew,
    Nix,
    Scoop,
    Winget,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnknownPackageManagerError;

impl FromStr for PackageManager {
    type Err = UnknownPackageManagerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "apt" => Ok(Self::Apt),
            "brew" => Ok(Self::Brew),
            "nix" => Ok(Self::Nix),
            "scoop" => Ok(Self::Scoop),
            "winget" => Ok(Self::Winget),
            _ => Err(UnknownPackageManagerError),
        }
    }
}
