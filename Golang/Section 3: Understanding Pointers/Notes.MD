# Understanding Pointers

## Understanding Pointers

- What are pointers?
  - Variables that store value addresses instead of values
  - The & symbol is how you get and store the value address
- Why Pointers?
  - Avoid Unnecessary Value copies
    - By default, Go creates a copy when passing values to functions
    - When a variable is passed as an argument, GO creates a copy of the value of that variable in a value address. The Go Garbage Collector then removes unused values from memory when function execution is finished
    - For very large & complex values, this may take up too much memory space unecessarily
    - Passing a pointer to a function, no copy is created, and the originally created value is used
  - Directly Mutate Values
    - Pass a pointer (address) instead of a value to a function
    - The function can then directly edit the underlying value - no return value is required
    - Can lead to less code (but also to less understandable code or unexpected behaviors)
- If you see a variable where the type is prefaced with a \* symbol, that means it's a pointer

## Pointers as Values

- If you want the value of a pointer, you would add a \* symbol where you pass a variable into a function

## A Pointer's Null Value

- Nil is a pointer's null value
- nil represents the absence of an address value

## Using Pointers & Passing Pointers to Functions

- You can't perform calculations on pointers in Go
- Instead you have to dereference the pointer with the \* symbol

## Using Pointers for Data Mutation
