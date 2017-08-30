extern crate regex;

use std::io;
use regex::Regex;

fn main() {
    // Getting user input
    let mut user_input = String::new();

    print!("Password: ");
    io::stdin().read_line(&mut user_input).expect(
        "Failed to read input.",
    );

    // Remove newline
    user_input.truncate(user_input.len() - 1);

    // Split by any char into array
    let split_regex = Regex::new(r"[A-Za-z]").unwrap();
    // Create a vector with the individual chacaters
    let string_array: Vec<&str> = user_input.split(split_regex).collect();

    // Matching regex for constanants
    let regex = Regex::new(r"aeiouAEIOU").unwrap();
    let up_regex = regex.to_uppercase();

    // Loop through each char, checking if regex matches
    for (index, &item) in user_input.as_bytes().iter().enumerate() {
        // If matches lowercase
        if regex.is_match(item[index]) {
            let mut letter_o: &str;
            if item[index] == item[index].to_uppercase() {
                letter_o = "O"
            } else {
                letter_o = "o"
            }
            // If it's a constanant, add an 'o' and itself, as per the challenge
            // e.g. an 'a' would result in the index being 'aoa'
            item[index] += letter_o + item[index];
        }
    }
}
