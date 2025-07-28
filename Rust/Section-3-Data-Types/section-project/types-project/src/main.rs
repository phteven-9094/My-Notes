/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.√

Cast the i32 to an i16 integer and assign the result
to a separate variable. √

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision. √

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean. √

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar. √

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar. √

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation. √

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() {
    let integer_value: i32 = 13_37; // Task 1 complete
    let integer_value_i16: i16 = integer_value as i16; // Task 2 complete

    let floating_point_value: f64 = 3.14159;
    println!("Floating point value: {:.3}", floating_point_value); // Task 3 complete

    let with_milk: bool = true;
    let with_sugar: bool = true; // Task 4 complete

    let is_my_type_of_coffee: bool = with_milk && with_sugar; //Task 5 complete

    let is_acceptable_coffee: bool = with_milk || with_sugar; // Task 6 complete

    let int_array: [i8; 4] = [1, 2, 3, 4];
    println!("Array: {:?}", int_array); // Task 7 complete

    let new_tuple: (i32, f64, bool, [i8; 4]) = (
        integer_value,
        floating_point_value,
        is_my_type_of_coffee,
        int_array,
    );
    println!("Tuple: {:?}", new_tuple); // Task 8 complete
}
