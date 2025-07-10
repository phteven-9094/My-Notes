# Introducing Functions

- Check exercises folder for several examples

## Introduction

- Reusable procedures
- Functions allows us to write reusable, modular code
- We define a "chunk" of code that we can then execute at a later time

```Javascript
function funcName() {
    //do something
}

function singSong() {
    console.log("DO");
    console.log("RE");
    console.log("MI");
}

singSong(); // Executes the function
```

## Arguments

- Essentially these are inputs to a function
- Allows the function to accept inputs and change the procedures of the function based on the argument
- Multiple arguments can be passed into a function as well

```Javascript
function rollDie(numSides) {
    randomNumber = Math.floor(Math.random() * numSides) + 1;
}

let diceRoll = rollDie(12); //Passes 12 as the number of sides of the Die to be rolled

function greet(firstName, lastName) { // These work like positional arguments in Python
    console.log(`Hey there, ${firstName} ${lastName}!`);
}
```

## The Return Keyword

- Similar to console.log() but instead of outputting to the console, it stores the value for later use

```Javascript
function addNums(x, y) {
    return x + y;
}

let total = addNums(2, 4) //saves 2+4 to the total variable

```
