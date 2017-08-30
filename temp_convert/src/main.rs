/// # Temperature Converter
/// This program asks for a temperature, its type (C or F), and converts it to the opposite.
/// Two inputs are required: the temperature and the "type" (aka C or F).
/// The program will continue to run until you ctrl+C

use std::io;
use std::f64;

fn main() {
    println!("-------------------");
    println!("Asks for a temperature, its type (C or F), and converts it to the opposite.");
    println!("-------------------");
    loop {
        let temperature: f64 = get_temp();
        let temp_type: String = get_type();
        let converted_temp: f64;
        let converted_to_show: String;

        println!("Your temp_type is '{}'", temp_type);

        // If the user entered Celcius, convert to F
        if temp_type == "c" || temp_type == "C" {
            // Stores the converted temperature
            converted_temp = convert_to_f(temperature);
            // Stores the opposite type of what the user entered to be displayed later
            converted_to_show = "F".to_string();
        // If user entered Farenheit, convert to C
        } else if temp_type == "f" || temp_type == "F" {
            // Stores the converted temperature
            converted_temp = convert_to_c(temperature);
            // Stores the opposite type of what the user entered to be displayed later
            converted_to_show = "C".to_string();
        // Error catcher
        } else {
            println!("Error, no type found!");
            break;
        }
        // Prints the converted temperatures, along with the types
        // Ex: "32C converts to 89.6F"
        // :.2 tells it to round to 2 decimal points
        println!(
            "{:.2}°{1} converts to {2:.2}°{3}",
            temperature,
            temp_type,
            converted_temp,
            converted_to_show
        );
    }
}

fn get_temp() -> f64 {
    println!("Input your temperature - number only!");
    loop {
        let mut usrinput_temperature = String::new();
        // Get the user input into the holder var
        io::stdin().read_line(&mut usrinput_temperature).expect(
            "Failed to read input.",
        );
        // Converts the String into f64
        let func_out: f64 = match usrinput_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // output
        return func_out;
    }
}

fn get_type() -> String {
    println!("Input either C or F");
    // Holder var for user input
    loop {
        let mut usrinput_type = String::new();
        // Get the user input into the holder var
        io::stdin().read_line(&mut usrinput_type).expect(
            "Failed to read input.",
        );
        // Quick check if it's not empty, could still fail if wrong type
        if usrinput_type != "" {
            // Output the string without newline
            return trunc_newline(usrinput_type);
        }
    }
}

fn trunc_newline(mut input: String) -> String {
    // removes 1 "char" from the string, aka the empty newline
    let len = input.len();
    input.truncate(len - 1);
    return input;
}

fn convert_to_c(f: f64) -> f64 {
    return ((f - 32.0) * 5.0) / 9.0;
}

fn convert_to_f(c: f64) -> f64 {
    return ((c * 9.0) / 5.0) + 32.0;
}
