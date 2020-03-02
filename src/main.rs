use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), io::Error> {
    let open_file = File::open("src/data/content.txt");

    let mut file = match open_file {
        Ok(f) => f,
        Err(e) => panic!("Error: {}", e),
    };

    let mut buffer = String::new();
    let contents = file.read_to_string(&mut buffer);

    match contents {
        Ok(file_content) => file_content,
        Err(e) => panic!("Error: {}", e),
    };

    println!("{}", buffer);

    Ok(())
}
