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

        Ok(username)
    }
}
