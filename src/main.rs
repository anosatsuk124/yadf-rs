mod platform;
mod spec;

use std::{ffi::OsString, path::Path, str::FromStr};

use clap::{Arg, Command};
use platform::Platform;
use spec::Spec;
fn main() {
    let cli = Command::new("yadf")
        .version("0.1.0")
        .about("")
        .subcommand(
            Command::new("cli").arg(
                Arg::new("platform")
                    .value_name("PLATFORM")
                    .short('p')
                    .long("platform")
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
                .get_one::<OsString>("file")
                .expect("A spec file is required."),
        ))),
        Some(("cli", args)) => Some(Spec::new(
            Platform::from_str(
                args.get_one::<String>("platform")
                    .expect("A platform option is required.")
                    .as_str(),
            )
            .unwrap(),
        )),
        _ => {
            eprintln!("There is no subcommands or options.");
            println!("Use --help option to check the command usage.");
            None
        },
    };

    println!("{:?}", spec);
}
