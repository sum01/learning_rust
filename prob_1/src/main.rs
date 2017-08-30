// #4 https://adriann.github.io/programming_problems.html
use std::{io, u64};
use std::time::SystemTime;
fn main() {
    // Get input
    let mut input = String::new();
    println!("Input a whole number > 1: ");
    io::stdin().read_line(&mut input).expect("Input failed.");

    // Convert to num
    let num: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => return,
    };

    if num <= 1 {
        println!("Num not > 1!");
        return;
    }

    // Start timer
    let start_time = SystemTime::now();
    // sum range
    let mut sum: u64 = 0;
    for x in 1..num {
        sum += x;
    }
    // Stop timer
    let end_time = start_time.elapsed().expect("Failed");

    println!(
        "\nSum of the range \"1 to {}\" = {1}.\nTook {2:?} seconds to calculate.\n",
        num,
        sum,
        end_time
    );
}
