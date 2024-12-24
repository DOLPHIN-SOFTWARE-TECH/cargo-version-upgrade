use clap::{Arg, Command};
use std::path::Path;

fn main() {
    let matches = Command::new("cargo-version-upgrade")
        .about("CLI tool to upgrade Cargo.toml version")
        .arg(
            Arg::new("increment")
                .help("Version increment type (patch, minor, major)")
                .required(true)
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("tags")
                .help("Optional prerelease tag (e.g., beta, alpha)")
                .long("tags")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    if let Err(e) = handle_command(&matches) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn handle_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let increment = matches.get_one::<String>("increment").expect("Required argument");
    let tag = matches.get_one::<String>("tags").map(|s| s.as_str());
    let cargo_toml_path = Path::new("Cargo.toml");

    cargo_version_upgrade::update_version(cargo_toml_path, increment, tag)?;
    Ok(())
}
