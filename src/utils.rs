use std::{
    fs::File,
    io::{self, Read},
};

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    let x = file.read_to_string(&mut contents)?;
    Ok(contents)
}
