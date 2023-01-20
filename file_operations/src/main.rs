use std::fs::File; // struct for the file
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running with arguments:");
    for argument in args.iter() {
        println!("- {argument}");
    }
    let mut filename = String::from("src/file.txt");
    if args.len() > 1 {
        filename = args[1].clone(); // cannot borrow, so we clone
    }

    let mut file = File::open(filename).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents");

    println!("File Contents:\n\n{}", contents);
}
