extern crate rand;
// f64 for big numbers & diff precision
use std::{u64, f64};
use rand::distributions::{IndependentSample, Range};
// Return a number between 1-6
fn get_roll() -> u64 {
    let between = Range::new(1, 6);
    let mut rng = rand::thread_rng();
    let sum = between.ind_sample(&mut rng);
    sum
}

// Get the difference between 2 numbers twice (both against a target num), then average the result
fn diff_3(tar_num: u64, num2: u64, num3: u64) -> f64 {
    let tar_num = tar_num as f64;
    let nums: [f64; 2] = [num2 as f64, num3 as f64];
    let len = nums.len() as f64;

    let mut diff: f64 = 0.0;
    for x in 0..nums.len() {
        diff += ((tar_num - nums[x]) / ((tar_num + nums[x]) / 2.0)) * 100.0;
    }

    let result = diff / len;
    result
}

fn inc_len(num: usize) -> usize {
    match num {
        0 => return 1,
        1 => return 2,
        3 => return 0,
        _ => return 0,
    };
}

fn main() {
    let players: [&str; 3] = ["Ben", "Sally", "George"];
    let mut p_scores: [u64; 3] = [0, 0, 0];
    let max_turns = players.len() - 1;
    let mut roll_1;
    let mut roll_2;
    let mut turn = 0;
    let mut roll_sum: u64;

    // Game loop (the number has no meaning..)
    for _ in 0..9999 {
        // Increment the turn for the correct player per-loop
        if turn >= max_turns {
            turn = 0;
        } else {
            turn += 1;
        }

        // Get the two rolls & add them up
        roll_1 = get_roll();
        roll_2 = get_roll();
        roll_sum = roll_1 + roll_2;

        // Add up the roll sum for the totals later
        p_scores[turn] += roll_sum;
    }

    // Print out totals
    let mut sum_diff: f64;

    for x in 0..players.len() {

        sum_diff = diff_3(
            p_scores[inc_len(x)],
            p_scores[inc_len(x + 1)],
            p_scores[inc_len(x + 2)],
        );
        println!(
            "{} summed {1} and diffed {2:.2}",
            players[x],
            p_scores[x],
            sum_diff
        );
    }
}
