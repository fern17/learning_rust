fn main() {
    let numbers1: [i32; 5] = [1, 2, 3, 4, 5];
    let numbers2 = [2; 20];

    println!("With iterator:");
    for n in numbers1.iter() {
        println!("{}", n);
    }

    println!("With index:");
    for i in 0..numbers2.len() {
        println!("{}", numbers2[i]);
    }
}
