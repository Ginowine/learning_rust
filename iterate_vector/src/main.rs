fn main() {
    let v = vec![100, 200, 300, 400, 500];

    for i in &v {
        println!("{i}");
    }

    // Iterating over mutable vector and adding some values to each element
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
}
