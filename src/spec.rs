use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::package_manager::PackageManager;
use crate::platform::Platform;
use crate::shell::Shell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    pub platform: Platform,
    pub shell: Shell,
    pub packages: Option<Vec<(PackageManager, String)>>,
    pub in_dir: PathBuf,
    pub out_dir: Option<PathBuf>,
    pub exclude: Option<Vec<PathBuf>>,
}

impl Spec {
    pub fn parse_yaml(path: &Path) -> Self {
        let mut file = File::open(path).expect("File not found.");
        let mut yaml = String::new();
        file.read_to_string(&mut yaml)
            .expect("Unsupported format. Check its encoding and format.");

        serde_yaml::from_str(&yaml).expect("It must be a correct yaml format.")
    }
}
