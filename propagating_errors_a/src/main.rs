#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;

        // This a shorter form of opening and reading a file
        // File::open("hello.txt")?.read_to_string(&mut username)?;

        // The standard library provides a shorter function that opens the file, creates a new String, 
        // reads the contents of the file, puts the contents into that String, and returns it.
        // fs::read_to_string("hello.txt")

        Ok(username)
    }
}
