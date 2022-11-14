fn main() {
    println!("64F => {}C", fahrenheit_to_celcius(64));    
}

fn fahrenheit_to_celcius(temperature: i32) -> f64{
    // formula to convert fahrenheit to celcius:
    (f64::from(temperature) - 32.0) * (5.0/9.0)
}