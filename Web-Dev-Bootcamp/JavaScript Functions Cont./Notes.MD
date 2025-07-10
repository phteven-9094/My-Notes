# JavaScript Functions Continued

- See other folders for examples

## Function Scope

- Variable Visibility
  - The location where a variable is defined dictates where we have access to that variable
    - If a variable is defined within a function, it's only accessible in that function unless it's returned
    - If a variable is defined outside a function (on it's own), it's accessible globally

### Block Scope

- If a variable is declared within a conditional block or loops, it will not be defined outside of it
- They can be accessed using the "var" keyword instead of "let" or "const", but you shouldn't be using the var keyword in general

### Lexical Scope

- If you have an inner function that is defined within an outer function, a variable declared in the outer function will be accessible in the inner function

## Function Expressions

- Involves storing a function in a variable

```Javascript
const square = function (num) {
    return num * num;
}
square(7);
```

## Higher Order Functions

- Functions that operate on/with other functions.
- They can:
  - Accept other functions as arguments
  - Return a function

## Defining Methods

- We can add functions as properties on objects.
- They're called methods

```Javascript
const math = {
    multiply : function (x,y) {
        return x * y;
    },
    divide : function(x,y) {
        return x / y;
    },
    square : function(x) {
        return x * x;
    }
}; // These would be called using the dot notation: math.multiply(5,5), math.divide(10,2), math.square(2)
```

- There is also shorthand available:

```Javascript
const myMath = {
    PI: 3.14159,
    square(num) {
        return num * num;
    },
    cube(num) {
        return num ** 3;
    }
}
```

## The "This" Keyword

- Use the keyword "this" to access other properties on the same object.

```Javascript
const person = {
    first:: 'Robert';,
    last: 'Herjavec';,
    fullName() {
        return `{this.first} ${this.last}`
    }
}
person.fullName(); // "Robert Herjavec"
person.last = "Plant";
person.fullName(); // "Robert Plant"


```

## Using Try/Catch

- Used for error handling
- Written similar to if, else
- Try is used to run code, but we anticipate that it could result in an error
- Catch is used if the try block fails and generally returns an error
- Finally can be used and runs code no matter what
