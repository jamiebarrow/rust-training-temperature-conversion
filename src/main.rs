use std::io;
use std::io::BufRead;

fn main() {
    println!("Enter temperature in Celcius: ");

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).expect("Could not read temperature!");
    
    let line = line.trim();
    let celcius: isize = match line.parse() {
        Ok(n) => n,
        Err(_) => 0
    };
    let fahrenheit = calculate_fahrenheit(celcius);
    println!("{} degrees Celcius = {} degrees Fahrenheit", celcius, fahrenheit);
}

fn calculate_fahrenheit(celcius: isize) -> isize {
    (celcius * 9 / 5) + 32
}
