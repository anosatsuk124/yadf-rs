use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum Shell {
    Fish,
    POSIX(POSIX),
    Nu,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum POSIX {
    Bash,
    Zsh,
}

#[derive(Debug)]
pub struct UnknownShellError;

impl FromStr for Shell {
    type Err = UnknownShellError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fish" => Ok(Self::Fish),
            "bash" => Ok(Self::POSIX(POSIX::Bash)),
            "zsh" => Ok(Self::POSIX(POSIX::Zsh)),
            "nu" => Ok(Self::Nu),
            _ => Err(UnknownShellError),
        }
    }
}
