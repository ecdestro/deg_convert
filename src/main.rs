use std::{io, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        interactive();
    } else if args.len() > 1 && args.len() != 3 {
        panic!("Invalid arguments: try converter 32 F");
    } else {
        let degrees: f32 = match args[1].parse() {
            Ok(v) => v,
            Err(_) => 0.0
        };
        let scale = &args[2].to_string();

        converter(degrees, scale.to_lowercase());
    }
}

fn interactive() {
    println!("Please enter a temperature and scale, like 32 F");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to get input");
    let temperature: Vec<&str> = user_input.trim()
        .split_whitespace()
        .collect();

    if temperature.len() != 2 {
        panic!("Invalid arguments, try 32 F");
    }

    let degrees = &temperature[0];
    let scale = &temperature[1].to_lowercase();
    match degrees.parse::<f32>() {
        Ok(num) => converter(num, scale.to_string()),
        Err(_) => panic!("{} is not a number.", degrees),
    };
}

fn converter(degrees: f32, scale: String) {
    if scale == "c" {
        let fahrenheit = ((9.00 / 5.00) * degrees) + 32.00;
        println!("{:.2} Celsius is {:.2} Fahrenheit", degrees, fahrenheit);
    } else if scale == "f" {
        let celsius = (degrees - 32.00) * (5.00 / 9.00);
        println!("{:.2} Fahrenheit is {:.2} Celsius", degrees, celsius);
    }
}
