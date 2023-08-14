fn main() {
    // Variables are inmutable by default
    let _inm = 5; // use underscore prefix for unused variable
    //_inm = 6; // this fails

    // Constants should be declared as UPPER_CASE and cannot be mutated
    const SECONDS: i8 = 60;
    println!("The value of seconds is {}", SECONDS);

    // mutability of variables
    let mut mutable_variable = 45;
    println!("The value of x is {}", mutable_variable);
    mutable_variable = 60; // By declaring it as mut, now this works
    println!("The value of x is {}", mutable_variable);

    // Datatypes
    let x: u64 = 45; // unsigned integer
    let f: f32 = 6.7; // floating point
    let b: bool = true; // boolean

    // Tuples
    let tup1 = (x, f, b, 50, (1, 2, 3)); // different types are possible
     
    println!("{}", tup1.2);
    println!("{}", (tup1.4).1); // nested tuple access

    // assign tuple values to separate variables
    let (ta, tb, tc) = tup1.4;
    println!("{ta}, {tb}, {tc}");

    let mut mutable_x = 10;
    let ref_to_x = &mut mutable_x; // when x is mutable bound, we cannot access it anymore until the reference is gone out of scope
    *ref_to_x += 1;
    println!("What was 10 is now: {ref_to_x}");

    // Strings 
    // - Are UTF8
    // - Are allocated in the heap
    // - Are growable
    // - Are not null terminated
    let mut my_string = String::from("How it is going?");
    let my_string2 = "Rust".to_string();
    println!("{my_string2}");
    println!("This is my string: {}", my_string);
    println!("Lenght: {}", my_string.len());
    println!("Is empty: {}", my_string.is_empty());
    println!("Split string:");
    for token in my_string.split_whitespace() {
        println!("{token}");
    }
    my_string.push_str(" Welcome to your tutorial");
    println!("{my_string}");

    // &str = string slice or stir
    let _str1 = "hello"; // &str type
}
