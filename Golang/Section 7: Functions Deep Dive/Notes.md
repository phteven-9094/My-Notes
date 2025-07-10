# Functions: Deep Dive

## Functions as Values & Function Types

- Functions are first-class values
  - This means you can use functions as arguments and return types
  - See transformNumbers() in functionsDeepDive/main.go

## Returns Functions as Values

- Functions can also be returned
- See getTransformerFunction() and transformedNumbers

## Introducing Anonymous Functions

- Anonymous Functions are used to declare a function within a function call
- I think it's similar to arrow functions in Javascript or lambdas in Python

## Making Sense of Recursion

- Recursion occurs when a function calls itself
- You must define an exit condition to avoid infinite recursive loops

## Using Variadic Functions

- Variadic functions are similar to using \*args in python
- This can be used with ...type
  - The ... represents that any amount of arguments can be used as long as they are of the required type

## Splitting Slices Into Parameter Values

- A trailing ... means it will unpack a slice of values and turn them into separate parameter values
