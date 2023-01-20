use std::fs::File; // struct for the file
use std::io::prelude::*;

fn main() {
    let mut file = File::open("src/file.txt").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents");

    println!("File Contents:\n\n{}", contents);
}
