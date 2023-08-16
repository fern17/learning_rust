# Rust notes
Multiple samples on how to write code with Rust
(tested under Windows 10 only)

## Resources
- Official documentation book: https://doc.rust-lang.org/book/
- Rust programming tutorials from dcode: https://www.youtube.com/@dcode-software
- A Firehose of Rust for busy people who know some C++: [https://www.youtube.com/watch?v=FSyfZVuD32Y]

## TODO
- lambdas and closures
- multi threading and mutexes
- trait system
- fat pointer

## Installation
### Windows:
Download and execute rustup-init.exe (x64) from https://www.rust-lang.org/tools/install

## General ideas of the language
References in Rust don't keep things alive. There is no garbage collector.
- Shared reference: `&t` (const pointer)
- Mutable (UNIQUE) reference: `&mut t` (non-const pointer)
- References are always valid, no dangling or null pointers
- Mutable references are unique
- Borrowing = taking a pointer
- Plain data types (i32, &T) have the `Copy` trait.
- By value operations on `Copy` types are bitwise copies (like in C)
- Other types are non-`Copy`. By-value operations in non-`Copy` types are moves, which are destructive bitwise copies.

## Project management
Cargo is Rust compilation and package manager. We use cargo to start a new project, build and run the program, and manage external libraries.
### Create a new project:
```
$ cargo new application_name --bin
```
(cannot start with digits)

`Cargo.toml` is the configuration file

### Compile with cargo:
```
$ cd application_name
$ cargo run
```

### Manual Compile: 
``` 
$ rustc main.rs
```

### Manual Run:
```
$ ./main.exe
```

### Generate documentation for the project and its dependencies:
```
$ cargo doc --open
```

## Input and Output
### Print into console:
```rust 
println!("Hello, world!");
```

Print variable:
```rust
println!("The value of x is {x}"); // (only if x is not an expression)
println!("The value of x+y is {}", x + y);
```
Print array:
```rust
let sv1: &[i32] = &my_vector1; // slice of a vector
println!("{:?}", sv1);
```

### Debug printing:
```rust
dgb!(*variable)
```

### Retrieving arguments:
```rust
let args: Vec<String> = env::args().collect();
println!("Running with arguments:");
for argument in args.iter() {
    println!("- {argument}");
}
```

### Reading input:
```rust
let mut v = String::new()
io::stdin() // return a handle to get input from the user
    .read_line(&mut v) // call and store in v (passed by reference), returns an enum Result type (OK or Err)
    .expect("Failed to read value"); // if read_line returned Err, run the expect part
```

## Variables
```rust
let x = 45;
```

By default, all variables are immutable
To make them mutable, use the `mut` keyword:
```rust
let mut x = 45;
```

### Mutable references:
```rust
let mut mutable_x = 10;
let ref_to_x = &mut mutable_x; // when x is mutable bound, we cannot access it anymore until the reference is gone out of scope
*ref_to_x += 1;
println!("What was 10 is now: {ref_to_x}");
```

There are some rules: any borrow must last for a scope no greater than that of the owner and you may have only one of:
 - one or more references `&T` to a resource
 - exactly one mutable reference `&mut T`

 ### Constants:
```rust
const LIMIT_NUMBER : i32 = 30; // (global scope)
```

## Datatypes:
```rust
let x: u64 = 45; // unsigned integer
let f: f32 = 6.7; // floating point
let b: bool = true; // boolean
```

### Tuples:
```rust
let tup1 = (x, f, b, 50, (1, 2, 3)); // different types are possible
    
println!("{}", tup1.2);
println!("{}", (tup1.4).1); // nested tuple access

// assign tuple values to separate variables
let (ta, tb, tc) = tup1.4;
    println!("{ta}, {tb}, {tc}");
```

### Strings:
#### Create a string type growable UTF-8 encoded:
```rust
let mut my_str = String::new();
```

#### Copy a string:
```rust
let s1 = "mystring".to_string();
let s2 = s1.clone();
```

#### String slice:
```rust
//&str = string slice or stir
let _str1 = "hello"; // &str type
``` 

### Enums:
```rust
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

#### Option enum:
It is a builtin with the following definition
```rust
enum Option<T> {
    None,
    Some(T)
}
```
To access the value, use match structure.

### Structures:
```rust
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
let bg = Color {red: 255, green: 70, blue: 50};
println!("BG: R: {} G: {} B: {}", bg.red, bg.green, bg.blue);
```

#### Add methods to a structure:
```rust
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

#### Traits of a structure:
Define a bundle of functions for structs to implement. 
One benefit of traits is you can use them for typing. 
You can create functions that can be used by any structs that implement the same trait.
They are also useful for composition.
```rust
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

#### Operator overloading
```rust
use std::ops::Add;
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T> 
    where T: Add<Output = T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

fn main() {
    let coord1 = Point {x: 5.0, y: 2.0};
    let coord2 = Point {x: 1.0, y: 3.0};
    let sum = coord1 + coord2;
    println!("{:?}", sum);
    }
```
### Arrays:
```rust
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

### Vectors:
```rust
let mut my_vector1: Vec<i32> = Vec::new();
let mut my_vector2 = vec![1,2,3,4];

my_vector1.push(42);
my_vector2.remove(1);

for number in my_vector1.iter() {
    println!("{}", number);
}

println!("my_vector1 = {:?}", my_vector1);
```
Note: when copying values of a vector, we need to use the `clone` method because we cannot borrow the individual values

### Hash maps:
```rust
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

### Hash sets:
```rust
use std::collections::HashSet;
fn main() {
    let mut hs = HashSet::new();
    hs.insert(10);
    hs.insert(20);
    hs.insert(30);

    hs.remove(&20);

    let mut hs2 = HashSet::new();
    hs2.insert(100);
    hs2.insert(10);
    hs2.insert(1000);
    
    println!("HS1: {:?}", hs);
    println!("HS2: {:?}", hs2);

    let intersection = &hs & &hs2;
    println!("Intersection: {:?}", intersection);

    println!("Union: {:?}", &hs | &hs2);
    println!("Difference: {:?}", &hs - &hs2);
}
```

## Ownership
For types implementing a move (heap)
```rust
let x = vec!["hello".to_string()];
let y = x; // move, x is no longer valid
let z = y.clone(); // clone: z has the same values as y (previously x). 
```
Cloning is an expensive operation because it has to duplicate all values.
However, for types implementing a copy (stack), we can copy them freely:
```rust
let x = 1;
let y = x; // x and y have the value
```

## Control structures
### If:
```rust
let n : i32 = 35;
let limit : i32 = 30;
if n < 30 {
    println!("The number is less than {}", limit);
} else {
    println!("The number is greater than {}", limit);
    }
```

### Infinite loop:
```rust
let mut n = 0;
'loop_label: loop {
    n += 1;
    
    if n == 7 {
        continue;
    }

    if n > 10 {
        break 'loop_label; // useful for nested loops
    }
    println!("n = {}", n);
}
```

### While loop:
```rust
let mut v = 1;
while v <= 50 {
    if v % 5 == 0 {
        println!("v = {}", v);
    }
    v += 1;
}
```

### For loop:
```rust
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

## Functions
Have to be named with snake case.

```rust
fn is_even(num: u32) -> bool {
    num % 2 == 0 // no semicolon => return value
}

fn concat_string(s: String) -> String{
    s + " World!"
}
```

## Lifetimes annotations
```rust
fn example_annotations<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    y
} // a is lifetime for x, b is lifetime for y. The return value has to match the lifetime of one of the parameters

let s: &'static str = "static lifetime that lives in the binary";
```

## Working with big projects
### Libraries:
Import a Library
```rust
use std::io;
```

### Crates:
A crate is a collection of Rust source code files
A project is a binary crate (executable)
To add a crate, list it in the dependencies section of the Cargo.toml file
```rust
[dependencies]
rand = "^0.8.5"
```

After adding a crate dependency, Rust will fetch and compile said crate
The fetched versions are also saved in the Cargo.lock file to ensure reproducible builds
If one wants to update the crates, one can run `cargo update`

### Modules:
```rust
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

## File handling
### Reading a file:
```rust
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
```rust
let mut contents = String::new();
let mut out_file = File::create("src/file_copy.txt").expect("Could not create file");
out_file.write_all(contents.as_bytes()).expect("Could not write to file");
```

## Other tools
### Match and option:
```rust
fn main() {
    find_occupation("Fernando");
    find_occupation("Joseph");
    find_occupation("Mary");
}

fn find_occupation(name: &str) { // returns an Option enum
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
### Random numbers:
```rust
use rand::Rng;
...
let secret_number = rand::thread_rng().gen_range(1..=100);  // generates a number in the current thread in [1, 100]
```


### Useful traits:
#### Drop:
Called when the variable goes out of scope => destructor
```rust
impl Drop for StructName {
    fn drop(&mut self) {

    }
}
```
#### Clone:
Make a copy of itself or from another object
```rust
impl Clone for StructName {
    fn clone(&self) -> Self
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}
```

#### Copy:
Only available for objects in the stack (not in the heap).

## Error handling:
- Use `RUST_BACKTRACE=1` to have the backtrace
- Unrecoverable errors: use the `panic!("msg")` macro to terminate the current thread.

The Result enum:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Error example:
```rust
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //panic!("Panicked here, exiting main thread");
    
    let file = File::open("error.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("cannot create the file"),
            }
            _ => panic!("it was some other error kind!"),
        }
    };

    //let file2 = File::open("error2.txt").expect("Error opening the file");
    
    let test = open_file();
    test.unwrap();

    rename_file().unwrap();
}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?; // propagates the error
    Ok(file)
}

fn rename_file() -> Result<(), Error> {
    let file = rename("error.txt", "renamed.txt")?;
    Ok(file)
}
}
```

