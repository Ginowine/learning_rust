fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Looping through a HashMap
    println!("{score}");

    for (key, value) in scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // updating value in a HashMap
    let mut scores_one = HashMap::new();
    scores_one.insert(String::from("Blue"), 10);
    scores_one.insert(String::from("Blue"), 25);

    println!("{:?}", scores_one);

    // Adding a Key and Value Only If a Key Isn’t Present

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


}
