fn main() {

    let word = String::from("Bigman");
    let latin = pig_latin(&word);
    println!("{latin}");
}

fn pig_latin(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut char_iter = word.chars();
    let first_letter = char_iter.next().unwrap();
    if vowels.contains(&first_letter) {
        format!("{}-hay", &word)
    } else {
        let remaining: String = char_iter.take(word.len() - 1).collect();
        format!("{}-{}ay", &remaining, first_letter)
    }
}
