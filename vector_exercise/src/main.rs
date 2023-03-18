use std::collections::HashMap;
use std::io;


fn mean (list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum)/(list.len() as f64)
}

fn median(numbers: &[i32]) -> f64 {
    let len = numbers.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&numbers[(mid - 1)..(mid + 1)])
    }else {
        f64::from(numbers[mid])
    }
}

fn mode(list: &[i32]) -> i32 {
    let mut occurrences: HashMap<&i32, i32> = HashMap::new();
    let mut max: (i32, i32) = (0, 0);

    for entry in list {
        let count = occurrences.entry(entry).or_insert(0);
        *count += 1;
    }

    for (&&key, &val) in &occurrences {
        if val > max.1 {
            max = (key, val);
        }
    }

    max.0
}

fn main() {
    //let mut numbers = vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 60];

    println!("Welcome to median and mode checker");
    println!("Type \"quit\" to end the program");

    loop {
        let mut list_input = String::new();

        println!("\nPlease input a list of integers you want stats about:\n(Format: 1,5,3,4,5) ");

        io::stdin()
            .read_line(&mut list_input)
            .expect("Failed to read line");

        let trimmed = list_input.trim();

        if trimmed == "quit" {
            break;
        }

        let mut list: Vec<i32> = trimmed
            .split(',')
            .map(|x| match x.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            })
            .collect();

        list.sort();

        println!("list: {:?}, mean: {}, median: {}, mode: {}",
        list,
    mean(&list),
    median(&list),
    mode(&list));
    }
}


