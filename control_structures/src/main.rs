enum Direction {
    Up, 
    Down,
    Left,
    Right
}

const LIMIT_NUMBER : i32 = 30;

fn main() {
    println!("If condition:");
    let number : i32 = 35;
    if number < LIMIT_NUMBER {
        println!("The number {} is less than {}", number, LIMIT_NUMBER);
    } else {
        println!("The number {} is greater than {}", number, LIMIT_NUMBER);
    }

    println!("Infinite loop");
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

    println!("While loop");
    let mut v = 1;
    while v <= 50 {
        if v % 5 == 0 {
            println!("v = {}", v);
        }
        v += 1;
    }

    println!("For loop");
    let numbers = LIMIT_NUMBER..51; // range => iterator
    for i in numbers {
        println!("i = {}", i);
    }
    // We can loop in vectors, but we need to use iter to obtain an iterator
    // With enumerate we obtain the index, and it returns a tuple
    let animals = vec!["Dog", "Cat", "Bird"];
    for (index, a) in animals.iter().enumerate() {
        println!("{}: {}", index, a);
    }

    println!("Enums and match (switch)");
    let player_direction : Direction = Direction::Up;
    match player_direction {
        Direction::Up => println!("We are heading up"),
        Direction::Down => println!("We are heading down"),
        Direction::Left => println!("We are heading left"),
        Direction::Right => println!("We are heading right"),
    }

    println!("Match and pattern matching");
    let number_to_match = 11;
    match number_to_match {
        1 => println!("This is one"),
        2 | 3 => println!("This is two or three"),
        4..=20 => println!("Between 4 and 20"),
        _ => println!("I dont know its value"),
    }
}
