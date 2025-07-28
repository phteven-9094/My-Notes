fn main() {
    // Integers
    let eight_bit: i8 = -112; // 8-bit signed integer
    let eight_bit_unsigned: u8 = 112; // 8-bit unsigned integer
    let sixteen_bit: i16 = -32_768; // 16-bit signed integer
    let sixteen_bit_unsigned: u16 = 65_535; // 16-bit unsigned integer
    let thirty_two_bit: i32 = -2_147_483_648; // 32-bit signed integer with underscore separators

    // Strings

    let string_literal = "Hello, world!"; // String literal
    let raw_string = r"This is a raw string"; // Raw string literal

    // Methods

    let value: i32 = -15;
    println!("{}", value.abs()); // Using the abs() method to get the absolute value

    let empty_space = "      my content     "; // String with leading and trailing spaces
    println!("{}", empty_space.trim()); // Using the trim() method to remove leading and trailing spaces

    println!("{}", value.pow(2)); // Using the pow() method to raise the value to the power of 2
    println!("{}", value.pow(3)); // Using the pow() method to raise the value to the power of 3

    // Floating Points
    let pi: f64 = 3.1415926535897932384; // 64-bit floating point number; It will start rounding the float around the 7 due to memory
    println!("The current value of pi is {pi}"); // Printing the value of pi

    println!("{}", pi.floor()); // Using the floor() method to round down the value of pi
    println!("{}", pi.ceil()); // Using the ceil() method to round up the value of pi
    println!("{}", pi.round()); // Using the round() method to round the value of pi to the nearest integer

    println!("{pi:.4}"); // Printing the value of pi with 4 decimal places

    let miles_away = 50;
    let miles_away_i8: i8 = miles_away as i8; // Casting the integer to an 8-bit signed integer

    let miles_away: f64 = 100.329032;
    let miles_away_int: i32 = miles_away as i32; // Casting the floating point number to a 32-bit signed integer

    // Math Operations
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 7;
    let division = 20 / 4;
    let remainder = 10 % 3;
    println!(
        "Addition: {}, Subtraction: {}, Multiplication: {}, Division: {}, Remainder: {}",
        addition, subtraction, multiplication, division, remainder
    );

    let floor_division = 5 / 3; // Integer division, result is 1
    let float_division = 5.0 / 3.0; // Floating point division, result is approximately 1.6667

    // Augmented Assignment Operator
    let mut year: i32 = 2025;
    year += 1; // Incrementing the year by 1
    println!("Next year is: {}", year); // Printing the incremented year
    year -= 5; // Decrementing the year by 5
    println!("Five years ago was: {}", year); // Printing the decremented year
    year *= 2; // Multiplying the year by 2
    println!("Double the year is: {}", year); // Printing the doubled year
    year /= 4; // Dividing the year by 4
    println!("A quarter of the year is: {}", year); // Printing a quarter of the year

    // Booleans
    let is_handsome: bool = true; // Boolean variable
    let is_silly: bool = false; // Another boolean variable

    let age: i32 = 21;
    let is_adult: bool = age >= 18; // Checking if the age is greater than or equal to 18
    println!("{} {}", age.is_positive(), age.is_negative()); // Checking if the age is positive or negative

    println!("{}", !true); // Using the NOT operator to negate a boolean value
    let can_see_rated_r_movie = age >= 17; // Checking if the age is greater than or equal to 17
    let cannot_see_rated_r_movie = !can_see_rated_r_movie; // Negating the previous condition

    // Characters
    let first_initial: char = 'S'; // Character variable
    let emoji: char = '😊'; // Emoji character

    println!(
        "{}, {}",
        first_initial.is_alphabetic(), // Returns True
        emoji.is_alphabetic()          // Returns False
    );

    // Arrays
    let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42]; // Array of integers

    let apples: [&str; 3] = ["Granny Smith", "McIntosh", "Red Delicious"]; // Array of strings
    println!("Lenghts: {}", apples.len()); // Printing the length of the apples array

    let currency_rates: [f64, 0] = []; // Empty array of floating point numbers

    let seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"]; // Array of seasons
    let first = seasons[0]; // Accessing the first element of the seasons array
    let second = seasons[1]; // Accessing the second element of the seasons array
    let thrid = seasons[2]; // Accessing the third element of the seasons array
    let fourth = seasons[3]; // Accessing the fourth element of the seasons array
    println!("The first season is {first}, the second is {second}, the third is {thrid}, and the fourth is {fourth}"); // Printing the seasons

    let mut mutable_array: [i32; 3] = [1, 2, 3]; // Mutable array of integers
    mutable_array[0] = 10; // Changing the first element of the mutable array
    println!("The first element of the mutable array is now: {}", mutable_array[0]); // Printing the first element of the mutable array


    // Display Trait
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    println!("{}", seasons); // Does not implement the Display trait, so this will not work
    println!("{:?}", seasons); // Using the Debug trait to print the seasons array
    println!("{seasons:#?}"); // Pretty printing the seasons array

    dbg!(2 + 2); // Using the dbg! macro to print the result of an expression and its location in the code
    dbg!(seasons); // Using the dbg! macro to print the seasons array and its location in the code

    // Tuples
    
    let employee: (&str, i32, &str) = ("Molly", 32, "Marketing"); // Tuple containing a string, an integer, and another string

    let name = employee.0; // Accessing the first element of the tuple
    let age = employee.1; // Accessing the second element of the tuple
    let department = employee.2; // Accessing the third element of the tuple

    let (name: &str, age: i32, department: &str) = employee; // Destructuring the tuple into separate variables


    println!("Employee Name: {}, Age: {}, Department: {}", name, age, department); // Printing the employee details

    // Ranges
    let month_days: Range<i32> = 1..31 // Range from 1 to 30 (exclusive)
    let month_days: RangeInclusive<i32> = 1..=31; // Range from 1 to 31 (inclusive)
    println!("{month_days:?}"); // Printing the range of days in a month

    for day: i32 in month_days {
        println!("Day: {}", day); // Iterating through the range and printing each day
    }

    let letters: Range<char> = 'a'..='z'; // Range of lowercase letters from 'a' to 'z'
    for letter in letters {
        println!("{}", letter); // Iterating through the range of letters and printing each letter
    }

    let colors = ["Red", "Green", "Yellow"];
    for color in colors {
        println!("{}", color); // Iterating through the array of colors and printing each color
    }

    // Generics
    let month_days: std::ops::Range<i32> = 1..31; // Range from 1 to 30 (exclusive) using generics
    let letters: std::ops::Range<char> = 'a'..='z'; // Range of lowercase letters from 'a' to 'z' using generics

}

