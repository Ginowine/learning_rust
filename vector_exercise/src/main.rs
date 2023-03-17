fn main() {
    let mut numbers = vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 60];

    fn mean (numbers: &[i32]) -> f64 {
        let sum: i32 = Iterator::sum(numbers.iter());
        f64::from(sum)/(numbers.len() as f64);
    }
}
