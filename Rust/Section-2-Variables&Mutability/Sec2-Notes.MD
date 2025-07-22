# Variables and Mutability

## Intro to Variables

- Declare a variable with "let"
- Varaibles follow snake_case_formatting
- A ";" is also used at the end of a line/command
- See Section-2-Variables&Mutability/variables_intro

## Interpolation with Curly Braces

- These work similarly to f-strings in Python
- See related section in Section-2-Variables&Mutability/variables_intro

## Underscore with Variables

- Underscores in front of a variable represent a variable that is purposefully unused
- You shouldn't have any unused variables, but this is for variables that haven't been used yet and you want to silence the warning

## Immutable and Mutable Variables

- Variables are immutable by default in Rust
- Adding the "mut" keyword to a variable definition will allow the variable to be mutable
- Important Note: Just because a variable is mutable, doesn't mean you can change the type of the variable!

## Rust Error Codes Index

- To see more information on an error, you can copy a command from a compiler error
- Example:
  - rustc --explain {error_code}
  - You can also use the Rust Error Codes Index site

## Variable Shadowing

- Means to redeclare a variable from scratch
- The original variable is "replaced" by the new one
- Is done by redeclaring a variable with the let keyword again
- Invalidates the previous declaration

## Scopes

- Variables are scoped to the block represented by {} that they live in

## Constants

- A constant is a name assigned to a value. A constant's value cannot change.
- Constants can be declared at any scope
- Essentially, can be used as a global variable
- Must be provided a type to use a constant
- Uses the const keyword and all caps for the name

## Type Aliases

- An alternate name that we can assign to an existing type.
- The type name should start with a capital letter
- Example
  - type Kilograms = f64
  - From here, you can use Kilograms instead of f64

## Compiler Directives

- An annotation that tells the compiler how to parse the source code
