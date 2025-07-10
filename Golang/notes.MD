# GO Essentials

## Organizing Code with Packages

- Every Go file must have a package statement at the beginning
- You can multiple files making up one package and you can have multiple packages in one application
- The import statement is importing a package from the Go Standard Library (https://pkg.go.dev/std)
- For other packages: https://pkg.go.dev

## The Importance of the Name "main"

- main is a "special" package name that tells Go that it is the main entry point of the application

## Understanding Go Modules and Building Go Programs

- Usually you run "go build" to create an executable file
  - The build command will look for a "main" module
  - You will need to create a module using "go mod init"
  - If you are planning on exposing it as a module library, you add a site to the end of the init command
    - go mod init example.com/first-app
  - This will create a go.mod file that created the module and then you can use the go build command
  - You can use go mod tidy to add dependencies
  - To run the executable that gets created, you can use "./app-name"
  - If you already have a module, you can simply use go run . to run the module to test it

## The "main" Function is Important!

- The main function must be named as much
- The main function will be called to run the file
- Most code will be in functions which will be called from within the main function so that Go knows where to start and what order to run code
- If you have multiple files within a package, there can only be one main function

## Working with Variables, Values, & Operators

- Variables are created with the "var" keyword
- Variables are generally named using camelCase
- Go is statically typed, so value types are important

## Understanding Value Types

- int = numbers without decimals
- float64 = numbers with decimals
- string = A text value (created with double quotes or backticks)
- bool = true or false
- You can change the type of a variable by wrapping it in a "function" of that type
  - See investment_calculator.go
- Special Types:
  - uint = strictly non-negative integer
  - int32 = a 32-bit signed integer (between -2,147,483,648 and 2,147,483,647)
  - rune = alias for int32, represents a Unicode code point and is used when dealing with Unicode
  - uint32 = 1 32-bit unsigned integer that can represent values from 0 to 4,294,967,295
  - int64 = a 64-bit integer, ranges from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
- Null Values
  - int => 0
  - float64 = 0.0
  - string = ""
  - bool = false

## Type Conversions & Explicit Type Assignment

- If we know we need to use values as specific types, you can do so at variable assignment
  - var varName type = value
- Sometimes you will still need to do type conversions, it's just important to know you can explicitly set variable types

## Using Alternative Variable Decalation Styles

- Instead of declaring a variable with the var keyword, you can use :=
  - This is essentially a shortcut you can use to declare and assign a value where the type should be inferred by Go
- You can also define multiple variables in the same line
- In short, use var with specific type assignment, use := when the type is inferred

## Making Sense of Constant Values

- The const keyword is used to declare a constant

## User Input Fetching

- You can grab user input using fmt.Scan()
- You need to assign a Pointer to a variable using the & symbol before a variable
- If you will be getting user input for a variable, you just assign a variable without setting a value to it
- To tell the user what kind of input:
  - Use fmt.Print() with a message in the parenthesis
  - See investment_calculator

## fmt.Scan() Limitations

- fmt.Scan() does not (easily) allow for multi-word input values. We'll learn about more complex input values later on.

## Formatting Strings Basics

- You can add strings to Println:
  - fmt.Println("Future value: ", futureValue)
- To format strings:
  - fmt.Printf("Future Value: %v", futureValue)
  - %v represents a value of a variable
- The standard library documentation goes further into how you can format strings

## Formatting Floats in Strings

- Use %f to format a decimal
- You can also control how many decimal places by putting a number before f
  - %.2f

## Creating Formatted strings

- To create a new string and store it in a variable
  - fmt.Sprint()
  - fmt.Sprintf()
  - fmt.Sprintln()

## Building Multiline Strings

- To create a multiline string:
  - Use backticks instead of double quotes
  - I don't really like this option for shorter text

## Understanding Functions

- Typically created below the main function
- Created with the func keyword
- You must declare the type of the parameters that the function takes in

## Functions: Return Values and Variable Scope

- You can return return values with the return keyword
- You can also return multiple values separated by a comma
- If a variable is declared in a function, they are only scoped to that function
- You have to define what the expected return types are for a function

## An Alternative Return Value Syntax

- Instead of the return keyword for multiple variables, you can define them before the return type declaration in the function
- Since you created the variables in the function creation, you can just use return without values

## Introducing "if" Statements & Booleans

- && is used to use an AND statement
- || is used for an OR statement
- If statement syntax:
  - if choice == 1 {
    }

## Working With "else if"

- else if is declared after the closing bracket of the if statement
- if choice == 1 {

}else if choice == 2 {

}

## Using "else"

- else is the final part of an if statement (if necessary)
- else is added similarly to else if

## Nested "if" Statements & Using "return" to Stop Function Execution

- You can create an if statement normally, and nest another if statement within the brackets
- The return keyword can be used to exit the nested if and stop the function

## Repeating Code with "for" Loops

- Go does not have an explicit while loop
- While loops can still be achieved with the for loop
- For loops are created with:
  - for iterable := 0; condition; code {}
  - Example:
    - for i := 0; i < 2; i++ {}

## Infinite Loops, "break" & "continue"

- You can achieve a while loop with an infinite loop and a break statement
- This can be done by defining the loop with just for {}
- You can also use the return keyword to exit a program
- The continue keyword continues the current iteration of a for loop without exiting the program

## Conditional For Loops

- You can also use a condition in a for loop
- for someCondition {}
- This can yeild a boolean value so the code continues to execute until the condition is false

## Making Sense of "switch" Statements

- Switch statements are for multiple if/else statements

  - ex

    - ```Go
        switch choice {
        case 1:
          code
      }

      ```

- break statements won't work the same with switch statements; you'll want to use the return statement to break out of a switch statement

## Writing To Files

- See writeBalanceToFile from bank.go

## Reading From Files

- See readBalanceFromFile from bank.go

## Handling Errors

- Most functions will return a 2nd value if an error can occur

## Time to Panic!

- The panic function is used in cases where you just can't continue and it will exit the application.
