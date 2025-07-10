# JavaScript Loops

- See various folders for examples

## Introduction to For Loops

- Loops allow us to repeat code
  - Print "hello" 10 times
  - Sum all numbers in an array
- Multiple Types:
  - For
  - While
  - for...of
  - for...in
- For Loop Syntax:

```Javascript
for (
    [initialExpression];
    [condition];
    [incrementExpression]
)

for (let i = 1; i <= 10; 1++) {
    console.log(i);
}
```

## Looping Over Arrays

- To loop over an array, start at index 0 and continue looping until last index (length-1)

```Javascript
const animals = ['lions', 'tigers', 'bears']

for (let i = 0; i < animals.length; i++) {
    console.log(i, animals[i]);
}
///Returns
//0 'lions'
//1 'tigers'
//2 'bears'

```

## Nested Loops

- You can nest loops to iterate over 2 sets within the same array

```Javascript
for (let i = 1; i <= 10; i++) {
    console.log(`i is: ${i}`) //First iteration, would list 1 through 10
    for (let j = 1; j < 4; j++) {
        console.log(`     j is: ${j}`) // 2nd iteration that occurs at each step of the first iteration
    }
}
```

## The While Loop

- While loops continue running as long as the test condition is true
- Basic syntax:

```Javascript
let num = 0;
while (num < 10){
    console.log(num);
    num++;
}
```

## The Break Keyword

- Used to break out of a loop (typcially a while loop)

```Javascript
let targetNum = Math.floor(Math.random() * 10);
let guess = Math.floor(Math.random() * 10);

while (true) {
    guess = Math.floor(Math.random() * 10);
    if (guess === targetNum) {
        console.log(`CORRECT! Guessed: ${guess} & target was $(targetNum)`);
        break;
    else {
        console.log(`Guessed ${guess}...Incorrect`);
    }
    }
}

```

## The For..Of Loop

- An easy way of iterating over arrays (or other iterable objects)
- NOTE: NO INTERNET EXPLORER SUPPORT
- Works much closer to the For Loops in Python

```Javascript
for (variable of iterable) {
    statement;
}

const numbers = [1,2,3,4,5,6,7,8,9];
for (let num of numbers) {
    console.log(num * num);
}
```
