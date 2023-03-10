use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // generates a number in the current thread in [1, 100]

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        /*
        io::stdin()git
            .read_line(&mut guess)
            .expect("Failed to read line");
        */
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {},
            Err(e) => println!("Failed to read line: {}", e),
        }

        // shadowing in guess
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break; 
            }
        }
    }
}
