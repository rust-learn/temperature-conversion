use std::io;

fn main() {
    loop {
        println!("Please select the conversion type:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        let mut conversion_type = String::new();
        io::stdin().read_line(&mut conversion_type)
            .expect("Failed to read line");
        let conversion_type = conversion_type.trim();
        let conversion_type = match conversion_type {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please input 1 or 2!");
                continue;
            }
        };

        println!("Please input the temperature:");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a valid temperature");
                continue;
            }
        };

        let converted_temperature = if conversion_type == 1 {
            (temperature - 32.) / 1.8
        } else {
            temperature * 1.8 + 32.
        };
        println!("The converted temperature is {}", converted_temperature);
    }
}
