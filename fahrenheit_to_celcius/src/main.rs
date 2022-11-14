use std::io;

fn main() {
    loop{
        println!("Please, enter the temperature in Fahrenheit to convert to celcius");

        let mut input_value = String::new();
    
        io::stdin()
            .read_line(&mut input_value)
            .expect("You have not entered a valid number");
    
        let input_value: f32 = input_value.trim().parse::<f32>().expect("Please type a number");
        let celcius: f32 = (input_value - 32.0) * 5.0 / 9.0;
    
        println!("the temperature in celcius is: {celcius}");
    }
}
