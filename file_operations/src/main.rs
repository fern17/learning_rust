use std::fs::File; // struct for the file
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running with arguments:");
    for argument in args.iter() {
        println!("- {argument}");
    }
    let mut in_filename = String::from("src/file.txt");
    if args.len() > 1 {
        in_filename = args[1].clone(); // cannot borrow, so we clone
    }
    let mut out_filename = String::from("src/file_copy.txt");
    if args.len() > 2 {
        out_filename = args[2].clone();
    }

    let mut in_file = File::open(in_filename).expect("Can't open file");
    let mut contents = String::new();
    in_file.read_to_string(&mut contents)
        .expect("Failed to read contents");

    println!("File Contents:\n\n{}", contents);

    let mut out_file = File::create(out_filename).expect("Could not create file");
    out_file.write_all(contents.as_bytes()).expect("Could not write to file");
}
