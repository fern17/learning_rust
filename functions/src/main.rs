fn main() {
    let n : u32 = 10;
    
    {
        // isolated codeblock
        println!("All numbers:");
        print_numbers_to(n);
    }
    
    {
        // another codeblock
        println!("Even numbers:");
        print_even_numbers(n);
    }
}

fn print_numbers_to(num: u32) {
    for n in 1..num {
        println!("{n}");
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}

fn print_even_numbers(num: u32) {
    for n in 1..num {
        if is_even(n) {
            println!("{n}");
        }
    }
}