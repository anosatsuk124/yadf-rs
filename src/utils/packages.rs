use crate::package_manager::PackageManager;
use std::{io, process::Command};

pub fn install_package(manager: PackageManager, packages: Vec<String>) -> io::Result<()> {
    match manager {
        PackageManager::Apt => {
            for package in packages {
                Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .arg(package)
                    .status()?;
            }
        }
        PackageManager::Nix => {
            panic!("Nix has not been supported yet!");
        }
        PackageManager::Brew => {
            for package in packages {
                Command::new("brew").arg("install").arg(package).status()?;
            }
        }
        PackageManager::Scoop => {
            panic!("Scoop has not been supported yet!");
        }
        PackageManager::Winget => {
            panic!("Winget has not been supported yet!");
        }
    }
    Ok(())
}
