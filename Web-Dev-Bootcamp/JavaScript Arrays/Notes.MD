# JavaScript Arrays

## Introduction

- An ordered collection of values:
  - list of comments on IG post
  - Collection of levels in a game
  - Songs in a playlist
- Semi-similar to lists in Python
- Arrays can also store other arrays inside of them
- Syntax:

```Javascript
let students = []; //Makes an empty array

let colors = ['red','orange','yellow']; //An array of strings

let lottoNums = [19,22,56,12,51]; //An array  of numbers

let stuff = [true, 68, 'cat', null]; //A mixed array
```

- Indexed just like a string where each element has it's own index

```Javascript
let colors = ['red','orange','yellow'];
colors[0] // 'red'
colors[1] // 'orange'
colors[2] // 'yellow'
colors[3] = 'blue';//Adds 'blue' to the array at the 3rd index
colors[2] = 'purple'; // Changes 'yellow' to 'purple'

```

## Push & Pop

- Push - add to end

```Javascript
let colors = ['red','orange','yellow'];
colors.push('grey'); // Add grey to the end of the array
```

- Pop - Remove from end

```Javascript
let colors = ['red','orange','yellow'];
colors.pop(); // Removes the last index of the array
```

## Shift & Unshift

- Shift - remove from start

```Javascript
let colors = ['red','orange','yellow'];
colors.shift(); // Removes the first index of the array
```

- Unshift - add to start

```Javascript
let colors = ['red','orange','yellow'];
colors.unshift('green'); // Adds green as the first index of the array

```

## Concat, indexOf, includes, & reverse

- concat - Merge arrays

```Javascript
let array1 = [1,2,3];
let array2 = [4,5,6];
let array3 = array1.concat(array2); // Creates an array of both arrays into 1
```

- includes - look for a value
  - Returns a boolean value

```Javascript
let colors = ['red','orange','yellow'];
colors.includes('red'); // Returns true
```

- indexOf - just like string.indexOf

```Javascript
let colors = ['red','orange','yellow'];
colors.indexOf('red'); // Returns 0
```

- join - creates a string from an array

```Javascript
let colors = ['red','orange','yellow'];
colors.join(); // Returns "red, orange, yellow"
```

- reverse - reverses an array

```Javascript
let colors = ['red','orange','yellow'];
colors.reverse(); // Returns the array backwards NOTE: THIS IS DESTRUCTIVE AND CHANGES THE ORIGINAL ARRAY
```

## Slice & Splice

- slice - copies a portion of an array

```Javascript
let colors = ['red','orange','yellow', 'green', 'blue', 'purple'];
colors.slice(0, 2); //Copies the first 3 elements of the array
colors.slice(1); // Copies from index of 1 until the end
colors.slice(1, 3); // Copies orange up to green, but doesn't include green
```

- splice - removes/replaces elements
  - Syntax convention:
    - array.splice(start, amountofitemstodelete, optional replacement)

```Javascript
let colors = ['red','orange','yellow', 'green', 'blue', 'purple'];
colors.splice(1, 0, 'maroon'); // Inserts maroon at index 1
```
