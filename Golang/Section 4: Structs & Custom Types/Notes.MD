# Structs and Custom Types

## Which Problem do Structs Solve?

- Structs are structures that allow you to group related values into one single value

## Defining a Struct Type

- Create a new type with the type keyword
- Name the type
- End with the type of the type, in this case "struct"
  - type user struct {
    data
    }

## Instantiating Structs & Struct Literal Notation

- Create a variable for your struct
- Assign the values to each value in the struct
- see structs.go

## Passing Struct Values as Arguments

- You can pass the struct values using the value type
- See outputUserDetails in structs.go

## Structs & Pointers

- Add an asterisk to function declaration to expect a pointer to a struct
- Use the ampersand symbol in an argument passed to an argument to create the pointer

## Introducing Methods

- Methods are functions created within a struct
- See the func under the User struct in structs.go

## Mutation Methods

- If you use a method to mutate a value created from a struct, you will need to use a pointer

## Using Creation/Constructor Functions

- A Constructor is a utility function is a function that takes care of creating a struct
- Usually follows a convention of using new at the beginning of the name
- see newUser()
- Constructors can also be used for validation, so that way you don't have to repeat validation steps when you create a new instance of a struct

## Structs, Packages, & Exports

- Generally speaking, you should not give access to struct values with capital letters and instead use a constructor function that gets exported

## Struct Embedding

- Struct does not have inheritance
- Instead you can simulate it with a new struct, see Admin struct in user.go and NewAdmin function for creating a struct while inheriting another struct

## Creating Other Custom Types and Adding Methods

- You can use the type keyword to create multiple types such as an alias to other built in types
- example

  ```Go
  type str string

  func (text str) log() {
    fmt.Println(text)
  }

  func main() {
    var name str = "Stephen"
    name.log()
  }
  ```

## Understanding Struct Tags

- Essentially meta data that you can add to your struct fields
- You add it at the end of the creation of a value with backticks ``
