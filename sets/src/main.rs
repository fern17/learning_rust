
use std::collections::HashSet;
fn main() {
    let mut hs = HashSet::new();
    hs.insert(10);
    hs.insert(20);
    hs.insert(30);

    println!("HS: {:?}", hs);

    hs.remove(&20);

    for x in hs.iter() {
        println!("Iter: {}", x);
    }

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
