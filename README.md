# Rust notes
Multiple samples on how to write code with Rust
(tested under Windows 10 only)

## Installation:
Download and execute rustup-init.exe (x64) from https://www.rust-lang.org/tools/install

## Common ideas:
References in Rust don't keep things alive. There is no garbage collector.
- Shared reference: &t (const pointer)
- Mutable (UNIQUE) reference: &mut t (non-const pointer)
References are always valid, no dangling or null pointers
Mutable references are unique
Borrowing : taking a pointer

## Project management 
### Create a new project
`$ cargo new application_name --bin`
(cannot start with digits)

`Cargo.toml` is the configuration file

### Compile with cargo
```
$ cd application_name
$ cargo run
```

### Manual Compile: 
`$ rustc main.rs`

### Manual Run:
`$ ./main.exe`

### Generate documentation for the project and its dependencies:
`cargo doc --open`

## Input and Output
### Print into console:
`println!("Hello, world!");`
Print variable:
`println!("The value of x is {x}");`  (only if x is not an expression)
`println!("The value of x+y is {}", x + y);`

### Debug printing:
dgb!(*variable)

### Retrieving arguments:
```
let args: Vec<String> = env::args().collect();
println!("Running with arguments:");
for argument in args.iter() {
    println!("- {argument}");
}
```

## Reading input:
```
let mut v = String::new()
io::stdin() // return a handle to get input from the user
    .read_line(&mut v) // call and store in v (passed by reference), returns an enum Result type (OK or Err)
    .expect("Failed to read value"); // if read_line returned Err, run the expect part
```

## Variables:
`let x = 45;`
By default, all variables are immutable
To make them mutable, use the `mut` keyword:
`let mut x = 45;`

Mutable references:
```
let mut mutable_x = 10;
let ref_to_x = &mut mutable_x; // when x is mutable bound, we cannot access it anymore until the reference is gone out of scope
*ref_to_x += 1;
println!("What was 10 is now: {ref_to_x}");
```
There are some rules: any borrow must last for a scope no greater than that of the owner and you may have only one of:
 - one or more references (&T) to a resource
 - exactly one mutable reference (&mut T)

Constants:
`const LIMIT_NUMBER : i32 = 30;` (global scope)

Datatypes:
```
let x: u64 = 45; // unsigned integer
let f: f32 = 6.7; // floating point
let b: bool = true; // boolean
```

## Control structures
### If
```
let n : i32 = 35;
let limit : i32 = 30;
if n < 30 {
    println!("The number is less than {}", limit);
} else {
    println!("The number is greater than {}", limit);
    }
```

### Infinite loop
```
let mut n = 0;
loop {
    n += 1;
    
    if n == 7 {
        continue;
    }

    if n > 10 {
        break;
    }
    println!("n = {}", n);
}
```

### While loop:
```
let mut v = 1;
while v <= 50 {
    if v % 5 == 0 {
        println!("v = {}", v);
    }
    v += 1;
}
```

### For loop:
```
let numbers = 30..51; // range => iterator
for i in numbers {
    println!("i = {}", i);
}
// We can loop in vectors, but we need to use iter to obtain an iterator
// With enumerate we obtain the index, and it returns a tuple
let animals = vec!["Dog", "Cat", "Bird"];
for (index, a) in animals.iter().enumerate() {
    println!("{}: {}", index, a);
}
```

### Enums:
```
enum Direction {
    Up, 
    Down,
    Left,
    Right
}

let player_direction : Direction = Direction::Up;
match player_direction {
    Direction::Up => println!("We are heading up"),
    Direction::Down => println!("We are heading down"),
    Direction::Left => println!("We are heading left"),
    Direction::Right => println!("We are heading right"),
}
```

Match and option:
```
fn main() {
    find_occupation("Fernando");
    find_occupation("Joseph");
    find_occupation("Mary");
}

fn find_occupation(name: &str) {
    println!("Occupation for {}: {}", name, match get_occupation(name) {
        Some(occ) => occ,
        None => "Occupation not found",
    });
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Fernando" => Some("Software engineer"),
        "Joseph" => Some("Taxi driver"),
        _ => None,
    }
}
```

## Tuples:
```
let tup1 = (x, f, b, 50, (1, 2, 3)); // different types are possible
    
println!("{}", tup1.2);
println!("{}", (tup1.4).1); // nested tuple access

// assign tuple values to separate variables
let (ta, tb, tc) = tup1.4;
    println!("{ta}, {tb}, {tc}");
```

## String:
Create a string type growable UTF-8 encoded
`let mut my_str = String::new();`

## Libraries:
`use std::io;`

## Crates:
A crate is a collection of Rust source code files
A project is a binary crate (executable)
To add a crate, list it in the dependencies section of the Cargo.toml file
```
[dependencies]
rand = "^0.8.5"
```
After adding a crate dependency, Rust will fetch and compile said crate
The fetched versions are also saved in the Cargo.lock file to ensure reproducible builds
If one wants to update the crates, one can run `cargo update`

Random numbers:
```
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..=100);  // generates a number in the current thread in [1, 100]
```

## Structures:
```
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
let bg = Color {red: 255, green: 70, blue: 50};
println!("BG: R: {} G: {} B: {}", bg.red, bg.green, bg.blue);
```

### Add methods to a structure:
```
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool{
        self.width == self.height
    }
}
```

### Traits of a structure:
Define a bundle of functions for structs to implement. 
One benefit of traits is you can use them for typing. 
You can create functions that can be used by any structs that implement the same trait.
They are also useful for composition.
```
struct Person {
    name: String,
    age: u8
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}
```

## Arrays:
```
let numbers1: [i32; 5] = [1, 2, 3, 4, 5];
let numbers2 = [2; 20];

for n in numbers1.iter() {
    println!("{}", n);
}

for i in 0..numbers2.len() {
    println!("{}", numbers2[i]);
}
}
```

## Vectors:
```
let mut my_vector1: Vec<i32> = Vec::new();
let mut my_vector2 = vec![1,2,3,4];

my_vector1.push(42);
my_vector2.remove(1);

for number in my_vector1.iter() {
    println!("{}", number);
}
```
Note: when copying values of a vector, we need to use the `clone` method because we cannot borrow the individual values

Hash maps:
```
use std::collections::HashMap;
fn main() {
    let mut marks = HashMap::new();
    marks.insert("Rust".to_string(), 59);
    marks.insert("C++".to_string(), 96);
    marks.insert("Python".to_string(), 65);
    println!("How many marks are there? {}", marks.len());
    
    match marks.get("C++") {
        Some(mark) => println!("{}: {}", "C++", mark),
        None => println!("You don't know {}", "C++")
    }

    marks.remove("Python");
    for (subject, mark) in &marks {
        println!("For {} you got {}", subject, mark);
    }

    println!("Did you study C++? {}", marks.contains_key("C++"));
```

## Modules:
```
mod my_mod {
    pub fn print_external() {
        println!("Printing from the exterior");
    }
    pub fn print_internal() {
        internal_function();
    }
    fn internal_function() {
        println!("Printing from the interior");
    }
    pub mod nested_module {
        pub fn print_external() {
            println!("Printing from the nested module");
        }
    }
}

mod module_in_file; 
/*
module_in_file.rs contents:
pub fn print_another_file() {
    println!("Printing from another source file");
}
*/

fn main() {
    my_mod::print_external();
    my_mod::print_internal();
    //my_mod::internal_function(); // fails, not visible
    my_mod::nested_module::print_external();
    module_in_file::print_another_file();
}
```
## Files:
### Reading a file:
```
use std::fs::File; // struct for the file
use std::io::prelude::*;

fn main() {
    let mut file = File::open("src/file.txt").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents");
    println!("File Contents:\n\n{}", contents);
}
```

### Writing to a file:
```
let mut contents = String::new();
let mut out_file = File::create("src/file_copy.txt").expect("Could not create file");
out_file.write_all(contents.as_bytes()).expect("Could not write to file");
```


## Resources:
Official documentation book: https://doc.rust-lang.org/book/
Rust programming tutorials from dcode: https://www.youtube.com/@dcode-software

## TODO: 
- annotations and lifetimes