/// "Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
/// taking advantage of the repetition in the song."

// Full lyrics for reference
/*
    On the first day of Christmas
    my true love sent to me:
    A Partridge in a Pear Tree

    On the second day of Christmas
    my true love sent to me:
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the third day of Christmas
    my true love sent to me:
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the fourth day of Christmas
    my true love sent to me:
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the fifth day of Christmas
    my true love sent to me:
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the sixth day of Christmas
    my true love sent to me:
    6 Geese a Laying
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the seventh day of Christmas
    my true love sent to me:
    7 Swans a Swimming
    6 Geese a Laying
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the eighth day of Christmas
    my true love sent to me:
    8 Maids a Milking
    7 Swans a Swimming
    6 Geese a Laying
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the ninth day of Christmas
    my true love sent to me:
    9 Ladies Dancing
    8 Maids a Milking
    7 Swans a Swimming
    6 Geese a Laying
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the tenth day of Christmas
    my true love sent to me:
    10 Lords a Leaping
    9 Ladies Dancing
    8 Maids a Milking
    7 Swans a Swimming
    6 Geese a Laying
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the eleventh day of Christmas
    my true love sent to me:
    11 Pipers Piping
    10 Lords a Leaping
    9 Ladies Dancing
    8 Maids a Milking
    7 Swans a Swimming
    6 Geese a Laying
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree

    On the twelfth day of Christmas
    my true love sent to me:
    12 Drummers Drumming
    11 Pipers Piping
    10 Lords a Leaping
    9 Ladies Dancing
    8 Maids a Milking
    7 Swans a Swimming
    6 Geese a Laying
    5 Golden Rings
    4 Calling Birds
    3 French Hens
    2 Turtle Doves
    and a Partridge in a Pear Tree
*/

fn main() {
    // Used to change the beginning snippet of the "Partridge and a Pear Tree" line
    let mut last_line;
    // Start at 1 so we don't have to add 1 to everything
    // Each loop is a "day" in the song
    for count in 1..13 {
        // Gets the word for the current "day"
        let day: &str = num_to_word(count);
        // These lines always stay the same
        println!("On the {} of Christmas", day);
        println!("my true love sent to me:");

        // All but the first day
        if count > 1 {
            // Prints the lyrics for each day, backwards
            for inner_count in 1..count {
                let current_lyric;
                match inner_count {
                    1 => current_lyric = "Turtle Doves",
                    2 => current_lyric = "French Hens",
                    3 => current_lyric = "Calling Birds",
                    4 => current_lyric = "Golden Rings",
                    5 => current_lyric = "Gees a Laying",
                    6 => current_lyric = "Swans a Swimming",
                    7 => current_lyric = "Maids a Milking",
                    8 => current_lyric = "Ladies Dancing",
                    9 => current_lyric = "Lords a Leaping",
                    10 => current_lyric = "Pipers Piping",
                    11 => current_lyric = "Drummers Drumming",
                    _ => current_lyric = "ERROR",
                };
                println!("{} {1}", count, current_lyric);
            }

            // Edits in the correct ending line
            last_line = "and a";
        } else {
            // Should only run once, on the first day
            last_line = "A";
        }
        println!("{} Partridge in a Pear Tree", last_line);
        println!(""); // linebreak for spacing
    }
}

// Doesn't infer lifetime from anywhere, so it must be 'static
fn num_to_word(num: u8) -> &'static str {
    let temp_hold;
    match num {
        1 => temp_hold = "first",
        2 => temp_hold = "second",
        3 => temp_hold = "third",
        4 => temp_hold = "fourth",
        5 => temp_hold = "fifth",
        6 => temp_hold = "sixth",
        7 => temp_hold = "seventh",
        8 => temp_hold = "eight",
        9 => temp_hold = "ninth",
        10 => temp_hold = "tenth",
        11 => temp_hold = "eleventh",
        12 => temp_hold = "twelfth",
        _ => temp_hold = "ERROR", // Catch-all to quiet compiler, shouldn't ever run
    };
    return temp_hold;
}
