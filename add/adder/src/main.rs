use add_one;
use add_two;
fn main() {
    let num = 10;
    let num1: i32 = 5;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    println!("Hello, world! {num1} plus two is {}!", add_two::add_two(num1));

}