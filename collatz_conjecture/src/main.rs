// https://www.reddit.com/r/ProgrammingPrompts/comments/1zum9i/collatz_conjecture/
use std::{io, u64};
fn input() -> u64 {
    let mut usr_input = String::new();
    println!("Input a number: ");
    io::stdin().read_line(&mut usr_input).expect(
        "Failed to read input",
    );
    let len = usr_input.len();
    usr_input.truncate(len - 1);
    usr_input.parse::<u64>().unwrap()
}
fn main() {
    let mut num: u64 = input();
    let mut count: u64 = 0;
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
        count += 1;
        println!("{}", num);
    }
    println!("Took {} steps", count);
}
