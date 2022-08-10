use std::{fs::File, io::Read, path::Path};

use crate::platform::Platform;

#[derive(Debug)]
pub struct Spec {
    platform: Platform,
}

impl Spec {
    pub fn new(platform: Platform) -> Self {
        Self { platform }
    }

    pub fn parse_yaml(path: &Path) -> Self {
        let mut file = File::open(path).expect("File not found");
        let mut yaml = String::new();
        file.read_to_string(&mut yaml);
        Self {
            platform: Platform::OSX,
        }
    }
}
