fn main() {
    // Challenge number
    let mut num = 31337357;

    println!("\nUsing starting number: {}\n", num);

    while num != 1 {
        // "If the number is divisible by 3, divide it by 3."
        if num % 3 == 0 {
            num /= 3;
        } else {
            // "If it's not, either add 1 or subtract 1 (to make it divisible by 3), then divide it by 3."
            if (num + 1) % 3 == 0 {
                num += 1;
            } else {
                num -= 1;
            }
            num /= 3;
        }

        println!("{}", num);
    }

    println!("Done!");
}
