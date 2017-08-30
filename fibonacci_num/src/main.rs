extern crate rprompt; // Used for simplified grabbing of user input.

use std::f64; // Needed for potential floating fibonacci.

fn main() {
    println!("---------");
    println!("This program gets the fibonacci number for the Nth number you supply.");
    println!("---------");

    loop {
        println!("Enter a whole number to calculate the fibonacci number for: ");
        let reply: String = rprompt::read_reply().unwrap();
        // nth has to be i32, powi doesn't accept i64
        let nth_num: i32 = string_to_i32(reply);
        let fibonacci: f64 = calc_fibonacci(nth_num);
        println!("The fibonacci number for {} is {1}", nth_num, fibonacci);
    }
}

fn calc_fibonacci(nth: i32) -> f64 {
    // Source for formulas, as I don't pretend to be a mathemitician..
    // http://www.maths.surrey.ac.uk/hosted-sites/R.Knott/Fibonacci/fibFormula.html#section1

    // _u is upper-case, _l is lower-case, in reference to the linked HTML's phi and Phi
    // sqrt_of_five var is only to avoid repeated typing of that lengthy sqrt call
    let sqrt_of_five: f64 = (5.0 as f64).sqrt() as f64;
    let phi_u: f64 = (sqrt_of_five + 1.0) / 2.0;
    let phi_l: f64 = phi_u - 1.0;

    // Magical formula I don't understand.
    // Thanks Binet (and the site I linked) :)
    let fib_out: f64 = (phi_u.powi(nth) - (-phi_l).powi(nth)) / sqrt_of_five;

    return fib_out;
}

fn string_to_i32(input: String) -> i32 {
    loop {
        let func_out: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return func_out;
    }
}
