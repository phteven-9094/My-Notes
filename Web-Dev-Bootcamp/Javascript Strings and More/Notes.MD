# Javascript Strings and More

- See stringsCheatSheet.js for examples

## Introducing Strings

- String of characters
- Represent text and must be wrapped in quotes
- Can use double or single quotes
  - Justremain consistent in codebase

## Indices and Length

- Each character in a string has a corresponding index (a positional number)
- Indices start at 0
  - If a string has 7 characters, the indices would be from 0 to 6
- Can be selected by notifying which index you want in brackets

```javascript
let animal = "Dumbo Octopus";
animal[0]; // "D"
animal[1]; // "u"
animal[6]; // "O"
```

- You can find the length of a string by adding .length to the variable

```javascript
animal.length;
```

- You can add or concatenate strings with a + sign

```javascript
"lol" + "lol";
//lollol
```

## String Methods

- Methods are built-in actions we can perform with individual strings
- Can help with things like:
  - Searching within a string (.indexOf())
  - Replacing part of a string (.replace('originalCharacter', 'replacingCharacter'))
  - Changing the casing of a string (.toUpperCase(), .toLowerCase())
  - Remove whitespace in a string (.trim())
  - Methods can be chained together as well
  ```javascript
  let hello = "   hello!";
  hello = hello.trim().toUpperCase();
  ```

## String Template Literals

- Similar to f strings in Python
- Uses backticks (``) instead of quotes
- Uses dollar signs to use a variable

```javascript
let product = "Artichoke";
let price = 2.25;
let qty = 5;
quote = `You bought ${qty} ${product}s. Total is ${price * qty}.`;
```

## Undefined and Null

- Null
  - "Intentionally absence of any value"
  - Must be assigned
- Undefined
  - Variables that do not have a value assigned

## Random Numbers and the Math Object

- Contains properties and methods for mathematical constants and functions
- Can be used by calling the Math object first

```javascript
Math.abs(); //Absolute value
Math.round(); // Rounds a number up
Math.PI(); //Pi
Math.pow(); //exponents
Math.floor(); //Removes decimal
Math.random(); //https://www.w3schools.com/js/js_random.asp
```

- There are a lot more options that can be seen via documentation or by calling Math in the browser
