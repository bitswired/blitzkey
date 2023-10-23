use clap::{arg, command, Command};
use std::{
    fs::File,
    io::{self, Read},
};

// --------------------------------

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    let x = file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn main() -> Option<String> {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("from-file").about("Adds files to myapp").arg(
                arg!([PATH])
                    .help("The name of the file to add")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("from-file", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH")?;
            println!("'myapp add' was used, name is: {:?}", path);
            read_file_to_string(path).ok()
        }
        _ => {
            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
        }
    }
}
