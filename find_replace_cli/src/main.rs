use text_colorizer::*;
use std::env;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!("{}: wrong number of arguments given. Expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }
}

fn print_help() {
    eprintln!("{} - replace a string with a new string", "Find and Replace".green()); // standard error printing
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}