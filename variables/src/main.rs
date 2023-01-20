fn main() {
    // mutability of variables
    let mut mutable_variable = 45;
    println!("The value of x is {}", mutable_variable);
    mutable_variable = 60;
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
}
