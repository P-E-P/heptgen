use clap::{arg, command, ArgMatches};
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    let file = File::open(filepath).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut declarations = vec![];
    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        if line.len() > 1 {
            match parser::function_declaration(&line) {
                Ok((_, dec)) => declarations.push(dec),
                Err(why) => eprintln!("Error while parsing line\n{}\n Error is: {:?}", &line, why),
            }
        }
    }

    println!("{:#?}", declarations);
}

fn validate_file_extension(filename: &Path, matches: &ArgMatches) -> bool {
    filename
        .extension()
        .map(|s| s == HEPTAGON_INTERFACE_EXTENSION)
        .unwrap_or(false)
        || matches.is_present("force")
}
