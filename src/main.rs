use std::fs::File;

fn main() {
    let f = File::open("src/data/content.txt");

    match f {
        Ok(_) => println!("💥"),
        Err(error) => println!("Open failed {:}", error),
    }
}
