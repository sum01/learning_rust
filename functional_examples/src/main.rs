use std::{io, env}; // io for input, env for startup params

fn main() {
    startup_parameters();
    user_input_examples();
    str_examples();
    range_examples();
    match_examples();
    array_examples();
    vector_examples();
    struct_examples();
    tuple_examples();
}

fn user_input_examples() {
    // An example user input thing
    fn get_input() -> String {
        let mut input = String::new();

        println!("Enter something: ");
        io::stdin().read_line(&mut input).expect(
            "Failed to read input.",
        );

        // Removing the newline from the input before returning it
        let len = input.len();
        input.truncate(len - 1);
        input
    }

    // The basic user input
    let string_input: String = get_input();
    println!("Input as string: {}", string_input);

    // Convert to num
    let converted_input: i32 = match string_input.trim().parse() {
        Ok(n) => n,
        Err(_) => return,
    };
    // Will only run if the match passes, since we return on Err
    println!("Input converted to i32: {}", converted_input);
}

fn str_examples() {
    // &str is faster/better than String, but cannot be used for user input (without some work)
    let str_test: &str = "Woowee this is a &str type";
    println!("&str type: {}", str_test);
}

fn range_examples() {
    // Rust can use ranges with the .. operator
    for range_x in 0..3 {
        println!("Range test: {}", range_x);
    }
}

fn array_examples() {
    // Array's are fixed sizes & all the same type
    let array: [i32; 5] = [0, 1, 2, 3, 4];
    println!("2nd index of array: {}", array[2]);

    // This declares multi_d_array as multi_d_array[2][5][2] with all positons = 0
    let multi_d_array = [[[0; 2]; 5]; 2];
    for m_array in multi_d_array.iter() {
        println!("{:?}", m_array);
    }
}

fn vector_examples() {
    fn type_example() -> Vec<String> {
        let bool_x: bool = true;
        let unsigned_x: u8 = 1;
        let signed_x: i8 = -1;
        // Return the vectorized data
        vec![
            bool_x.to_string(),
            unsigned_x.to_string(),
            signed_x.to_string(),
        ]
        // of course you could also do [16 32 64] types
        // float/unsigned 64 types require `use std::f64;` and `use std::u64;` respectively
    }

    // Just keeping input in brackets [] treats them as an array, so we use the vec! macro
    let vector_test: Vec<i32> = vec![0, 3, 5, 6];
    // Access the specific index
    println!("3rd index of vector: {}", vector_test[3]);
    // or access all of them iterating over the whole thing
    for vec_itr in vector_test.iter() {
        println!("vector_test: {}", vec_itr);
    }

    let mut mutable_vector: Vec<String> = type_example();
    // Adding another element to the vector
    mutable_vector.push("Testing".to_string());
    for vec_type in mutable_vector.iter() {
        println!("Vec type-test: {}", vec_type);
    }

    // Doing type _ makes the compiler choose, like let without : type
    let multi_dimensional_vector: Vec<_> = vec![vec![vec![123]]];
    for vec_multi in multi_dimensional_vector.iter() {
        println!("Multi-dimensional vec: {:?}", vec_multi);
    }
}

fn match_examples() {
    let match_test: u8 = 2;
    // Match is basically switch-case, but you can perform complex checks as well
    // the underscore is a catchall, which can be used either for bugs or as a "default"
    match match_test {
        0 | 1 | 2 => println!("Match: {}", match_test),
        _ => println!("ERR"),
    }
}

fn struct_examples() {
    // Essentially defines a data-type, which is accessed with `Example.x`
    // These are very similar to tuples, but are more "rigid" in decleration.
    struct Example {
        x: u8,
        y: i8,
        z: &'static str, // str has to have defined lifetime
    }

    let mut example_struct = Example {
        x: 3,
        y: -3,
        z: "hi",
    };
    println!(
        "x: {}, y: {1}, z: {2}",
        example_struct.x,
        example_struct.y,
        example_struct.z
    );
    example_struct.z = "bye";
    println!("z after changing: {}", example_struct.z);

    // Nested function that returns the Example struct with new values
    fn return_a_struct() -> Example {
        Example {
            x: 1,
            y: 0,
            z: "no",
        }
    }

    // reassign new vals via func
    example_struct = return_a_struct();
    println!(
        "After reassign of struct || x: {}, y: {1}, z: {2}",
        example_struct.x,
        example_struct.y,
        example_struct.z
    );
}

fn tuple_examples() {
    // Tuples are similar to array's, but their purpose is to hold different types of data
    // As for the difference between a tuple & vector, I'm not really sure...
    let tup_example = ("sample text", 1, 42.13, true, false);
    println!("Tuple: {:?}", tup_example);
    // Specific indexes can be reached via tuple.index
    println!("Tuple index 2: {}", tup_example.2);

    // NOTE: tuple.len() doesn't seem to work
}

fn startup_parameters() {
    // Example of passing startup parameters, which can be passed without --
    let mut args: Vec<_> = env::args().collect();
    // zero index isn't a user-passed value, so it should be removed (usually)
    args.remove(0);
    // Only bother to check passed startup parameters if there are any.
    if args.len() > 0 {
        println!("\nArgument(s) passed:");
        for x in args.iter() {
            println!("  {}", x);
        }
        println!("\n");
    }
}
