# Data Types

## Intro to Data Types

- Rust is a statically typed language, which means the compiler must know the types of all variables at compile time.
- The compiler can infer the types of variables based on their initial assignments
- See datatypes/src

### Scalar Types

- A scalar type is a type that holds a single value
- Rust has 4:

  - integers
    - Whole number
  - floating-point
    - Decimal number
  - Boolean
  - characters

    #### Integers

    - Signed
      - Support positive and negative values. They start with an "i"
    - Unsigned

      - Support zero and positive values. They can store a larger max value in the positive direction.
      - Start with a "u"

        ##### Bits

        - The number after the i or u refers to the amount of bits the integer needs in your computer's memory
        - A bit is the smallest unit of computer memory. It represents a value of either 0 or 1.
        - An i32 requires 32 bits of memory. An f64 requires 64 bits.

        ##### Using \_ as Visual Separator for Numbers

        - Instead of a comma, use \_ as a seperator for long numbers and the compiler will ignore them

        ##### usize and isize Types

        - usize and isize is used depending on the system
        - It is equivavelent to a u32 on a 32-bit computer and same with a u64 on a 64_bit system

    #### Strings

    - String literals are strings enclosed within quotes
    - There are ways to render a special character using a "\"
    - To escape a "\\", use a "\\"
    - You can also prefix a string with a "r" to represent a raw string without needing to escape characters

      ##### Methods

      - A method is a function that lives on a value. It's an action we can ask the value to execute.
      - Syntax -
        - value.method()

    #### Floating Point Types

    - 2 types:
      - 32 bit float
      - 64 bit float
      - Works similar to float and double in Java
      - There is no unsigned type like with integers
      - f64 is the default float inferred type
    - You can format floating points to customize the printed representation of the interpolated value using a :
      - Example
        - {pi:.4} Specifies the precision, so print out the value of pi but with only 4 digits after the decimal

    #### Casting Types with the "as" Keyword

    - This is a way of transforming a type into another

    #### Math Operations

    - See data_types/src

    #### Booleans

    - A type whose values can only be true or false

    #### Logic

    - "==" checks for equality
    - "!=" checks for inequality
    - "&&" AND Logic
    - "||" OR Logic

    #### The Character Types

    - Represents a single unicode character
    - Only uses single quotes rather than double quotes like on strings

### The Array Type

- A fixed-size collection of homogenous data (Data of the same type)
- Declared using [type;length] for the type declaration
- Declared with brackets similar to Lists in Python
- For reading and writing array elements:
  - see data_types/src/main.rs

### The Display Trait

- Traits will be covered in depth later on
- Imagine a non-specific contract witht he following requirements: "You promise to arrive at 9am at a location"
  - Situations can be different but they would honor the same promise
- A Trait is a contract that requires that a type support one or more methods.
- Traits establish consistency between types; methods that represent the same behavior have the same name
- When a type opts in to honoring a trait's requirements, we say the type implements the trait
- Type can vary in their implementation but still implement the same trait.
- A Type can choose to opting in to implementing a trait
- A type can implement multiple traits. There are hundreds of traits available in Rust.
- A trait is called an interface or protocol in other programming languages
- The Display trait requires that a type can be represented as a user-friendly, readable string
- The Display trait mandates a format method that returns a string
- When we use the {} interpolation syntax, Rust relies on the format method.
- Integers, floats, and booleans all implement the Display trait so we are able to interpolate them with curly braces.
- It is not always clear how a complex type should be represented as a piece of text.
- Not all types implement the Display trait. One example is the array type.

### The Debug Trait

- "Debug should format the output in a programmer-facing, debugging context"
- While array types don't implement the Display trait, they do implement the Debug trait

### The dbg! Macro

- Prints and returns the value of a given expression for quick and dirty debugging

### The Tuple Type

- Collection type that supports diffrent types for it's values
- Created with parenthesis

### Ranges and Range Iteration

- A range is a sequence/interval of consecutive values

### Generics

- A generic is a type argument.
- Imagine a box (a type), it's a special type because it can be used to store different types of items
- A placeholder for a future specific type
- Will be covered further in-depth later
