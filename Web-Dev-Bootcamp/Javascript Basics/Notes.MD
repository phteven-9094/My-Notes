# JavaScript Basics

## Primitives and The Console

- Primitive Types (basic building blocks):
  - Number
  - String
  - Boolean
  - Null
  - Undefined
- Starting off, we'll run code using the Chrome developer tools console. Then, we'll learn how to write external scripts.
- Check basics_cheatsheet.js for some examples

## JavaScript Numbers

- Number - One number type
  - Positive numbers, negatives, whole (ints), decimals

## What is NaN

- NaN - Not a Number
  - A numeric value that represents something that is not a number
  - Example:
    - 0/0 //NaN
    - 0/NaN //NaN

## Variables and Let

- Variables are like labels for values
  - We can store a value and give it a name so that we can:
    - Refer back to it later
    - Use that value to do stuff
    - Or change it later on
- Basic Syntax:
  - let someName = value;
    - let year = 1985;
    - Make me a variable called "year" and give it the value of 1985
- Anytime you are declaring a variable, you should have a semi-colon at the end
- Variable names should be short, but descriptive enough to know what the variable is used for/stores
- In Javascript, variable names are written in camelCase instead of snake_case in Python
- If you need to update a variable like one storing numbers and you want to update it by a certain amount, you can use different operations:
  ```javascript
  let score = 5; // assign 5 to score
  score = score + 5; // take score and add 5 to it
  score += 5; // take score and add 5 to it
  score -= 2; // take score and subtract 2
  score = score - 2; // take score and subtract by 2
  score *= 2; // multiply score by 2
  score /= 2; // divide score by 2
  score++; // Operator that first returns the original value and then increments the variable by 1
  ++score; // Operator that increments the variable first and then returns the value
  ```

## Const and Var

- const is a constant variable

  - a variable that can not be changed
  - Basic syntax:
    - const luckyNum = 26;

- var
  - The old variable keyword
  - This was used before let and const

## Booleans

- True or False values
  - let isLoggedIn = true;
  - let isLoggedIn = false;
