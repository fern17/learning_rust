use std::collections::HashMap;

fn find_mark(marks: &HashMap<String, i32>, subject: &str) {
    match marks.get(subject) {
        Some(mark) => println!("{}: {}", subject, mark),
        None => println!("You don't know {}", subject)
    }
}

fn main() {
    let mut marks = HashMap::new();
    marks.insert("Rust".to_string(), 59);
    marks.insert("C++".to_string(), 96);
    marks.insert("Python".to_string(), 65);
    println!("How many marks are there? {}", marks.len());
    
    find_mark(&marks, "C++");
    find_mark(&marks, "R");

    marks.remove("Python");
    for (subject, mark) in &marks {
        println!("For {} you got {}", subject, mark);
    }

    println!("Did you study C++? {}", marks.contains_key("C++"));
    println!("Did you study Python? {}", marks.contains_key("Python"));
}
