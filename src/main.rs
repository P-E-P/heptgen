use clap::{arg, command, ArgMatches};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use parser::Declaration;

mod parser;

const TYPE_TEMPLATE : &str = r#"
#ifndef {unit_name}_TYPES_H
#define {unit_name}_TYPES_H

{type_definition}

#endif /* {unit_name}_TYPES_H */
"#;

const HEADER_TEMPLATE : &str = r#"
#ifndef {unit_name}_H
#define {unit_name}_H

{type_inclusion}

{function_declarations}

#endif /* ! {unit_name}_H */
"#;

const C_TEMPLATE : &str = r#"
#include "{header_file}.h"

{function_definitions}

"#;

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
    let declarations = parse_declarations(file);

    println!("{:#?}", declarations);

    let unit_name = filepath.file_name().expect("Invalid file").to_string_lossy();

    for dec in declarations {}

}

fn parse_declarations(file: File) -> Vec<Declaration> {
    let reader = BufReader::new(file);
    let mut result = vec![];
    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        if line.len() > 1 {
            match parser::function_declaration(&line) {
                Ok((_, dec)) => result.push(dec),
                Err(why) => eprintln!("Error while parsing line\n{}\n Error is: {:?}", &line, why),
            }
        }
    }

    result
}

fn validate_file_extension(filename: &Path, matches: &ArgMatches) -> bool {
    filename
        .extension()
        .map(|s| s == HEPTAGON_INTERFACE_EXTENSION)
        .unwrap_or(false)
        || matches.is_present("force")
}
