use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(
            arg!([FILE])
        ).get_matches();

    println!("File is: {}", matches.value_of("FILE").expect("Missing file name"));
}
