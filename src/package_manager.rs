use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum PackageManager {
    Apt,
    Brew,
    Nix,
    Scoop,
    Winget,
    Snap(Vec<SnapOption>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SnapOption {
    Edge,
    Beta,
    Classic,
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
            "snap" => Ok(Self::Snap(Vec::new())),
            _ => Err(UnknownPackageManagerError),
        }
    }
}

impl ToString for SnapOption {
    fn to_string(&self) -> String {
        match self {
            Self::Classic => String::from("--classic"),
            Self::Edge => String::from("--edge"),
            Self::Beta => String::from("--beta"),
        }
    }
}

