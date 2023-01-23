fn main() {
    find_occupation("Fernando");
    find_occupation("Joseph");
    find_occupation("Mary");
}

fn find_occupation(name: &str) {
    println!("Occupation for {}: {}", name, match get_occupation(name) {
        Some(occ) => occ,
        None => "Occupation not found",
    });
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Fernando" => Some("Software engineer"),
        "Joseph" => Some("Taxi driver"),
        _ => None,
    }
}