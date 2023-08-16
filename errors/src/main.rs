
use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;
use std::fs::rename;

fn main() {
    //panic!("Panicked here, exiting main thread");
    
    let file = File::open("error.txt");
    
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("cannot create the file"),
            }
            _ => panic!("it was some other error kind!"),
        }
    };

    //let file2 = File::open("error2.txt").expect("Error opening the file");

    let test = open_file();
    test.unwrap();

    rename_file().unwrap();
}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?; // propagates the error
    Ok(file)
}

fn rename_file() -> Result<(), Error> {
    let file = rename("error.txt", "renamed.txt")?;
    Ok(file)
}
