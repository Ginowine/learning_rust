use std::collections::HashMap;
use std::io;

fn main() {
    let mut emp_list: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Select a number:");
        println!("[1] Add Employee");
        println!("[2] Retrieve a list");
        println!("[3] Quit");

        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("error");
        let response = response.trim();

        match response {
            "1" => {
                println!("Input Add <EMPLOYEE NAME> to <DEPARTMENT>");

                let mut emp_rec = String::new();
                io::stdin().read_line(&mut emp_rec).expect("error");
                let emp_rec = emp_rec.trim();
                let emp_rec_count = emp_rec.split_ascii_whitespace().count();
                let word_count: usize = 4;

                if emp_rec_count < word_count {
                    println!("Format should be 'Add <EMPLOYEE NAME> to <DEPARTMENT>'");
                    continue;
                }

                let add_word = emp_rec
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .to_ascii_lowercase();

                if add_word != "add" {
                    println!("'add' word missing as 1st word!");
                    println!("Format should be 'Add <EMPLOYEE NAME> to <DEPARTMENT>'");
                    continue;
                }

                let index_of_to_word = emp_rec
                    .split_ascii_whitespace()
                    .rev()
                    .position(|w| w.to_ascii_lowercase() == "to")
                    .unwrap_or_default();

                if index_of_to_word == 0 {
                    println!("'to' word missing between <EMPLOYEE NAME> and <DEPARTMENT>!");
                    println!("Format should be 'Add <EMPLOYEE NAME> to <DEPARTMENT>'");
                    continue;
                }

                // get employee name
                let index_of_to_word = emp_rec
                    .split_ascii_whitespace()
                    .position(|p| p.to_ascii_lowercase() == "to")
                    .unwrap();
                let emp_name: Vec<&str> = emp_rec
                    .split_ascii_whitespace()
                    .take(index_of_to_word)
                    .filter(|p| p.to_ascii_lowercase() != "add")
                    .collect();
                let emp_name = emp_name.join(" ");

                if emp_name.len() == 0 {
                    println!("Missing <EMPLOYEE NAME> between 'add' and 'to'");
                    println!("Format should be 'Add <EMPLOYEE NAME> to <DEPARTMENT>'");
                    continue;
                }

                // get department
                let index_of_to_word = emp_rec
                    .split_ascii_whitespace()
                    .position(|p| p.to_ascii_lowercase() == "to")
                    .unwrap();

                let dept: Vec<&str> = emp_rec
                    .split_ascii_whitespace()
                    .skip(index_of_to_word + 1)
                    .collect();
                let dept = dept.join(" ");

                // check if employee exist in department
                let employees_per_dept = emp_list.get(&dept);
                match employees_per_dept {
                    Some(_) => match employees_per_dept.filter(|p| p.contains(&emp_name)) {
                        Some(_) => {
                            println!("Employee already exist at {}!", dept);
                        }
                        None => {
                            emp_list
                                .entry(dept.to_ascii_lowercase())
                                .or_insert(Vec::new())
                                .push(emp_name);

                            println!("Successfully added employee!");
                        }
                    },
                    None => {
                        emp_list
                            .entry(dept.to_ascii_lowercase())
                            .or_insert(Vec::new())
                            .push(emp_name);

                        println!("Successfully added employee!");
                    }
                };
            }
            "2" => {
                println!("Select a number:");
                println!("[1] list of all people in a department");
                println!("[2] all people in the company by department, sorted alphabetically");
                println!("[3] Quit");

                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("error");
                let response = response.trim();

                match response {
                    "1" => {
                        println!(
                            "Input department to retrieve a list of all people in a department:"
                        );

                        let mut dept = String::new();
                        io::stdin().read_line(&mut dept).expect("error");
                        let dept = dept.trim().to_ascii_lowercase();

                        if let None = emp_list.get(&dept) {
                            println!("Department does not exist!");
                            continue;
                        }

                        println!("List of all people in {}:", dept.to_ascii_uppercase());
                        for values in emp_list.get(&dept).unwrap() {
                            println!("{}", values.to_ascii_uppercase());
                        }
                    }
                    "2" => {
                        println!("All people in the company by department, sorted alphabetically");

                        for (dept, emp_name) in emp_list.iter_mut() {
                            emp_name.sort();

                            println!(
                                "{} department employees:\n{}\n",
                                dept.to_ascii_uppercase(),
                                emp_name.join("\n").to_ascii_uppercase()
                            );
                        }
                    }
                    "3" => break,
                    _ => (),
                }
            }
            "3" => break,
            _ => (),
        }
    }
}