fn main() {

    let number_list = vec![30, 25, 35, 15, 50, 60, 80];
    let mut largest_number = &number_list[0];

    for number in &number_list{
        if number > largest_number{
            largest_number = number;
        }
    }
    println!("The largest number is {}", largest_number);
}
