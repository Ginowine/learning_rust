use std::fmt::format;

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

    // Appending a charatcer to a string
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // concatenating two strings
    let s1 = String::from("Gino");
    let s2 = String::from("wine");

    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let s = s1 + "-" + &s2 + "-" + &s3;

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // Slicing strings using range
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("{s}");

    // Iterating over a string
        for c in "Ginowine".chars() {
        println!("{c}");
    }
}
