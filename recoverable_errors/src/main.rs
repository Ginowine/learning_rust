use std::fs::File;

fn main() {

    let greeting_file_result = File::open("gino.txt");

    let result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("Hello, world!");
}
