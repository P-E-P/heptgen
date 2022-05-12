use clap::{arg, command, ArgMatches};
use std::path::Path;

mod parser;

const HEPTAGON_INTERFACE_EXTENSION: &str = "epi";

fn main() {
    let matches = command!()
        .arg(arg!([FILE]))
        .arg(arg!(-f - -force "Force file parsing"))
        .get_matches();

    let filepath = match matches.value_of("FILE") {
        Some(filename) => Path::new(filename),
        None => panic!("No file specified"),
    };

    if !validate_file_extension(filepath, &matches) {
        panic!("Invalid file extension");
    }
}

fn validate_file_extension(filename: &Path, matches: &ArgMatches) -> bool {
    filename
        .extension()
        .map(|s| s == HEPTAGON_INTERFACE_EXTENSION)
        .unwrap_or(false)
        || matches.is_present("force")
}
