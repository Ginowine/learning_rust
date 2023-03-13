fn main() {
    let data = "initial content";
    let s = data.to_string();

    // You can also initialise a string literal as below
    let s = "initial content".to_string();
    println!("{s}");

    // This is another way of creating a string from a string literal
    let s = String::from("Gino Wine");

    // The below code appends a string to a string
    let mut s = String::from("Gino Wine");
    s.push_str("Osahon");

    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("s1 is {s1}");
}
