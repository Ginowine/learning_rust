fn main() {
    const DAYS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "FIVE GOLD RINGS",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

(0..12).for_each(|day| {
    let day_with_suffix = day_with_suffix_maker(day);

    println!("\nOn the {day_with_suffix} day of xmas my true love gave to me:");

    (0..day + 1).rev().for_each(|item| {
        if day > 0 && item == 0 {
            print!("And ");
        }
        println!("{}", DAYS[item]);
    });
});

}

fn day_with_suffix_maker(day: usize) -> String {
    let suffix = match day + 1 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    (day + 1).to_string() + suffix
}
