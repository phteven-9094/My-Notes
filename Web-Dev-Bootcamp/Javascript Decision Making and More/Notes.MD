# Javascript Decision Making

- See Notes.md for notes and various other folders for examples

## Comparison Operators

- Example:
  - ">" - Greater than
  - < - Less than
  - ">=" - greather than or equal to
  - <= - less than or equal to
  - == - equality
  - != - Not equal
  - ==== - strict equality
  - !== - strict inequality

### Triple vs Double Equals

- Double equal signs (==)
  - Checks for equality of value, but not equality of type
  - It coerces both values to the same type and then compares them
  - This can lead to some unexpected results
- Triple equal signs (===)
  - Checks for equality of value AND type

## Console, Alert, and Prompt

- Console
  - console.log()
    - prints arguments to the console
    - similar to print in Python
- Alert
  - alert()
    - Creates a pop up to the user
- Prompt
  - prompt()
    - Creates a pop up to the user where the user can also provide an input

## Running Javascript from a Script

- Can run code in a file by defining it in the head or bottom of the body of an HTML file
- This depends on when you want the Javascript to load
- Generally you want the elements of HTML to load before the script

```HTML
<body>
    <script src="app.js"></script>
</body>
```

## If Statements

- Only runs code if given condition is true

```Javascript
let rating = 3;
if (rating ===3){
    console.log("YOU ARE A SUPERSTAR!");
}
```

### Else-If

- If not the first thing, maybe this other thing?
- Can have multiple else-ifs

```Javascript
let rating = 2;
if (rating ===3) {
    console.log("YOU ARE A SUPERSTAR!")
}
else if (rating === 2) {
    console.log("MEETS EXPECTATIONS!")
}
```

### Else

- If nothing else was true, do this
- Not always required

```Javascript
let rating = 2;
if (rating ===3) {
    console.log("YOU ARE A SUPERSTAR!")
} else if (rating === 2) {
    console.log("MEETS EXPECTATIONS!")
} else {
    console.log("NEEDS IMPROVEMENT")
}
```

## Nesting Conditionals

- Conditionals can be nested inside other conditionals as well

```Javascript
let password = "cat dog";
if (password.length >= 6) {
    if (password.indexOf(' ')!== -1) {
        console.log("Password cannot include spaces");
    } else {
        console.log("Valid Password");
    }
} else {
    console.log("Password too short");
}
```

## Truthy and Falsey Values

- All JS values have an inherent truthyness or falsyness about them:
  - Falsy values:
    - false
    - 0
    - "" (empty string)
    - null
    - undefined
    - NaN
  - Everything else is truthy

### Logical AND

- Returns true if both values are true
  - If one value is false, the whole thing is false
- Represented by &&

### Logical OR

- Returns true if either value is true
  - Only returns false if both are false
- Represented by ||

### Logical NOT

- Represented by !
- Put in front of other operations
  - !true
  - !==
  - !===

## Switch Statement

- The switch statement is another control-flow statement that can replace multiple if statements.

```Javascript
const day = 2;
switch(day) {
    case 1:
        console.log("Monday");
        break;
    case 2:
        console.log("Tuesday");
    case 3:
        console.log("Wednesday");
    case 4:
        console.log("Thursday"):
    case 5:
        console.log("Friday");
    default:
        console.log("Invalid Number");
}

```

- The above is read where switch(day) is the value being evaluated
- Each case is similar to if statements
  - case 1
    - if day == 1
- default is almost like an else statement where if nothing else is true, do the default
- Advice from StackOverflow:
  - "If you are switching on the value of a single variable, then I'd use a switch every time, it's what the construct was made for. Otherwise, stick with if-else statements"
