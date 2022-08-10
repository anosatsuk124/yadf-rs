use std::path::PathBuf;
use std::{path::Path, str::FromStr};

use clap::{Arg, Command};
use yadf::package_manager::PackageManager;
use yadf::platform::Platform;
use yadf::shell::{Shell, POSIX};
use yadf::spec::Spec;

const REPO_URL: &str = "https://github.com/anosatsuk124/yadf";

fn main() {
    let cli = Command::new("yadf")
        .version("0.1.0")
        .about("Yet Another Dotfiles manager")
        .author("Satsuki Akiba <anosatsuk124@gmail.com>")
        .subcommand(
            Command::new("cli")
                .arg(
                    Arg::new("platform")
                        .value_name("PLATFORM")
                        .short('p')
                        .long("platform")
                        .required(true),
                )
                .arg(
                    Arg::new("package_manager")
                        .value_name("PACKAGE_MANAGER")
                        .short('m')
                        .long("manager")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("yaml").arg(
                Arg::new("file")
                    .value_name("FILE")
                    .short('f')
                    .long("file")
                    .required(true),
            ),
        );
    let spec = match cli.get_matches().subcommand() {
        Some(("yaml", args)) => Some(Spec::parse_yaml(Path::new(
            &args
                .get_one::<String>("file")
                .expect("A spec file is required."),
        ))),
        Some(("cli", args)) => {
            eprintln!("This feature is not implemented. Please use other subcommands.");
            println!(
                "Check the newest news via this repository page: {}",
                REPO_URL
            );
            Some(Spec {
                platform: Platform::from_str(
                    args.get_one::<String>("platform")
                        .expect("A platform option is required.")
                        .as_str(),
                )
                .unwrap(),
                package_manager: PackageManager::from_str(
                    args.get_one::<String>("package_manager")
                        .expect("A package_manager option is required.")
                        .as_str(),
                )
                .unwrap(),
                shell: Shell::POSIX(POSIX::Bash),
                packages: vec![],
                dir: PathBuf::new(),
                exclude: vec![PathBuf::new()],
            })
        }
        _ => {
            eprintln!("There is no expected subcommands or options.");
            println!("Use --help option to check the command usage.");
            None
        }
    };

    println!("{:?}", spec);
}
