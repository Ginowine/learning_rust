fn main() {

    let mut s = String::from("Hello world");

    let word = first_word(&s);

    println!("The first word is: {}", word);

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);

    println!("The first word is: {}", word);

    let my_string = String::from("hello world");

    let word_one = first_word(&my_string[..]);

    println!("The first word is: {}", word_one);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);

    println!("The first word is: {}", word);

    let word = first_word(&my_string_literal[..]);

    println!("The first word is: {}", word);

    let word = first_word(my_string_literal);

    println!("The first word is: {}", word);

    s.clear();

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
