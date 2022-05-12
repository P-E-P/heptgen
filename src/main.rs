use clap::{arg, command, ArgMatches};
use parser::Declaration;
use std::fs::{File, write};
use std::io::{BufRead, BufReader};
use std::path::Path;

mod parser;

const TYPE_TEMPLATE: &str = r#"
#ifndef {unit_name}_TYPES_H
#define {unit_name}_TYPES_H

{type_definition}

#endif /* {unit_name}_TYPES_H */
"#;

const HEADER_TEMPLATE: &str = r#"
#ifndef {unit_name}_H
#define {unit_name}_H

#include "{types_file}_types.h"

{function_declarations}

#endif /* ! {unit_name}_H */
"#;

const C_TEMPLATE: &str = r#"
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

    let unit_name = filepath
        .file_stem()
        .expect("Invalid file")
        .to_string_lossy();

    let mut output_types: String = "".to_string();
    let mut function_declarations = vec![];

    for dec in declarations {
        // Collect output types
        let mut types = "".to_string();
        for var in dec.outputs {
            types.push_str(&String::from(var));
            types.push_str("; ");
        }
        output_types.push_str(&format!(
            "typedef struct {{ {} }} {}__{}_out;",
            types,
            capitalize(&unit_name),
            dec.name
        ));
        output_types.push_str("\n");

        // Collect function declarations
        let mut inputs = "".to_string();
        for var in dec.inputs {
            inputs.push_str(&String::from(var));
            inputs.push_str(", ");
        }
        function_declarations.push(format!(
            "void {}__{}_step({}{}__{}_out *_out)",
            capitalize(&unit_name),
            dec.name,
            inputs,
            capitalize(&unit_name),
            dec.name
        ));
    }

    let types_file = TYPE_TEMPLATE
        .replace("{type_definition}", &output_types)
        .replace("{unit_name}", &unit_name.to_uppercase());

    let header_file = HEADER_TEMPLATE
        .replace("{unit_name}", &unit_name.to_uppercase())
        .replace("{types_file}", &unit_name.to_lowercase())
        .replace(
            "{function_declarations}",
            &function_declarations
                .iter()
                .map(|s| s.to_owned() + ";\n")
                .collect::<Vec<String>>()
                .join(""),
        );

    let c_file = C_TEMPLATE
        .replace("{header_file}", &unit_name.to_lowercase())
        .replace(
            "{function_definitions}",
            &function_declarations
                .iter()
                .map(|s| s.to_owned() + "\n{\n\n}\n\n")
                .collect::<Vec<String>>()
                .join(""),
        );

    write(format!("{}_types.h", &unit_name.to_lowercase()), types_file)
        .expect("Unable to write type file");
    write(format!("{}.h", &unit_name.to_lowercase()), header_file)
        .expect("Unable to write header file");
    write(format!("{}.c", &unit_name.to_lowercase()), c_file).expect("Unable to write c file");
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
