//Constants
const TAX_RATE: f64 = 0.07; // Constant declaration

fn main() {
    let apples = 50; // Variable assignment
    let oranges = 14 + 6; // Expression evaluation
    let _fruits = apples + oranges; // Combining variables

    // Corresponds with the Interpolation with Curly Braces section within Sec2-Notes.MD

    println!("This year, my garden has {} apples.", apples); //Original way to do interpolation
    println!("This year, my garden has {apples} apples."); // Using variable directly in the string
    println!("This year, my garden has {apples} apples and {oranges} oranges."); // Multiple variables in interpolation
    println!("This year, my garden has {} apples and {} oranges.",apples, oranges); // Using multiple variables with positional arguments
    println!("This year, my garden has {0} apples and {1} oranges. I can't beleive I have {0} apples!", apples, oranges); // Using positional arguments with indices

    // Mutable vs Immutable Variables

    let gym_reps = 10;
    println!("I plan to do {gym_reps} reps today at the gym.");
    // gym_reps = 12; // Uncommenting this line will cause a compile-time error because gym_reps is immutable
    let mut gym_reps = 10; // Making the variable mutable
    gym_reps = 12; // Now we can change the value

    // Variable Shadowing
    let grams_of_protein = "100.345"; // Initial variable declaration
    let grams_of_protein = 100.345; // Shadowing the previous variable with a new type
    let grams_of_protein = 100; // Shadowing again with a different value

    // Variable Scopes
    {
        let inside_scope = "I am inside a block scope.";
        println!("{}", inside_scope); // This will work
    }
    // println!("{}", inside_scope); // Uncommenting this line will cause a compile-time error because inside_scope is not accessible here
    let outside_scope = "I am outside the block scope.";
    println!("{}", outside_scope); // This will work, as outside_scope is accessible here

    // Constants
    let income = 50000.0; // Variable declaration
    println!("The tax rate is {}.", TAX_RATE); // Using the constant TAX_RATE
    let tax = income * TAX_RATE; // Using the constant in a calculation
    println!("The tax on an income of ${} is ${}.", income, tax); // Displaying the result

    // Type Aliases
    type Kilograms = f64; // Creating a type alias for f64
    let weight: Kilograms = 70.5; // Using the type alias
    println!("My weight is {} kilograms.", weight); // Displaying the weight using the type alias

    // Compiler Directives
    #[allow(dead_code)] // This directive allows the code to compile even if there are unused variables
    let unused_variable = "This variable is not used anywhere."; // This variable won't cause a warning due to the directive

    
}