use std::io;

fn main() {
    let mut input = String::new();
    // Get user input
    println!("Enter a string to count: ");
    io::stdin().read_line(&mut input).expect(
        "Failed to read input",
    );

    let mut string_iter = input.split(" ");

    for x in string_iter.len() {
        println!("{}", string_iter[x].unwrap());
    }

    /*
    let word_count: usize = get_count(&our_string);

    println!(
        "There are {} word(s) in the string \"{1}\"",
        word_count,
        our_string
    );
    */
}
