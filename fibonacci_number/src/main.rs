use std::io;
fn main() {
    loop{
        println!("Type in a non negative number");

        // read input from user 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // converts input string to unsigned integer 
        let input: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // calls the fibonacci function and prints both the number entered and the fibonacci number 
        println!(
            "Fibonacci number ({}) => {}",
            input,
            fibonacci_number(input),
        );
    }
}

// function takes an argument and calculate the fibonacci number 
fn fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_number(n - 1) + fibonacci_number(n - 2);
    }
}
