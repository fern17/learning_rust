
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //panic!("Panicked here, exiting main thread");
    
    let file = File::open("error2.txt").expect("Error opening the file");

    //let file = match file {
    //    Ok(file) => file,
    //    Err(error) => match error.kind() {
    //        ErrorKind::NotFound => match File::create("error.txt") {
    //            Ok(file_created) => file_created,
    //            Err(err) => panic!("cannot create the file"),
    //        }
    //        _ => panic!("it was some other error kind!"),
    //    }
    //};
}
