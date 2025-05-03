use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Invalid arguments: try convert 32 F");
    }
    let degrees: f32 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => 0.0
    };
    let scale = &args[2];

    convert(degrees, scale);
}

fn convert(degrees: f32, scale: &String) {
    if scale == "c" || scale == "C" {
        let fahrenheit = ((9.00 / 5.00) * degrees) + 32.00;
        println!("{:.2} Celsius is {:.2} Fahrenheit", degrees, fahrenheit);
    } else if scale == "f" || scale == "F" {
        let celsius = (5.00 / 9.00) * (degrees - 32.00);
        println!("{:.2} Fahrenheit is {:.2} Celsius", degrees, celsius);
    }
}
