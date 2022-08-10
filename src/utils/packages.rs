use crate::package_manager::PackageManager;
use std::{io, process::Command};

pub fn install_package(packages: Vec<(PackageManager, String)>) -> io::Result<()> {
    for package in packages {
    match package {
        (PackageManager::Apt, p) => {
                Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .arg(p)
                    .status()?;
        }
        (PackageManager::Nix, _) => {
            panic!("Nix has not been supported yet!");
        }
        (PackageManager::Brew, p) => {
                Command::new("brew").arg("install").arg(p).status()?;
        }
        (PackageManager::Scoop, _) => {
            panic!("Scoop has not been supported yet!");
        }
        (PackageManager::Winget, _) => {
            panic!("Winget has not been supported yet!");
        }
        (PackageManager::Snap(op), p) => {
                Command::new("sudo")
                    .arg("snap")
                    .arg("install")
                    .arg(p)
                    .args(op.into_iter().map(|op| op.to_string()))
                    .status()?;
        }
    }
    }
    Ok(())
}
