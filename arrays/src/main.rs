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

    // vectors
    let mut my_vector1: Vec<i32> = Vec::new();
    let mut my_vector2 = vec![1,2,3,4];
    
    my_vector1.push(42);
    my_vector2.remove(1);

    for number in my_vector1.iter() {
        println!("{}", number);
    }
    for number in my_vector2.iter() {
        println!("{}", number);
    }
}
