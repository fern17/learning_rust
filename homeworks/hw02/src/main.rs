
fn concat_string(s: String) -> String{
    s + " World!"
}

fn control_flow(i: i32) {
    if i == 1 {
        println!("value is one");
    }
    else if i > 50 {
        println!("value is greater than 50");
    }
    else if i < 25 {
        println!("value is less than 25");
    }
    else {
        println!("value is between 25 and 50");
    }
}

fn main() {

    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{val1} % {val2} = {ans}");

    let mut v = vec![2, 4, 6, 8, 10];
    println!("vec = {:?}", v);
    v.remove(4);
    println!("vec = {:?}", v);
    v.push(12);
    println!("vec = {:?}", v);

    let s = String::from("Hello");
    let s2 = concat_string(s);
    println!("s2 = {:?}", s2);

    control_flow(1);
    control_flow(25);
    control_flow(55);
    control_flow(30);
}
