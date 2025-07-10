# Callbacks & Array Methods

- Callback Function
  - Any function that is passed as an argument to another function so that it can be executed in that other function

## The forEach Method

- Accepts a callback function
- Calls the function once per element in the array
- This has been semi-replaced by the for loop (for (let el of numbers))

```Javascript
const nums = [9,8,7,6,5,4,3,2,1];

nums.forEach(function (n){
    console.log(n * n)
}); //prints each number in the array squared

nums.forEach(function (el) {
    if (el % 2 === 0) {
        console.log(el)
    }
}); //prints each even number
```

## The map Method

- Creates a new array with the results of calling a callback on every element in the array

```Javascript
const texts = ['rofl', 'lol','omg','ttyl'];
const caps = texts.map(function (t) {
    return t.toUpperCase();
})

texts; //['rofl', 'lol','omg','ttyl']
caps; //['ROFL', 'LOL','OMG','TTYL']
```

## Intro to Arrow Functions

- Shorter syntax for writing function expressions
- Does not work with Internet Explorer
- Works the same as function expressions, just shorter syntax

```Javascript
const square = (x) => {
    return x * x;
}

const sum = (x,y) => {
    return x + y;
}

```

### Arrow Function Implicit Returns

- Some situations where you don't need to include the return keyword
- Will only work if you are doing just one instruction

```Javascript
const isEven = num => (
        num % 2 === 0
); // Notice the parens instead of curly braces

const add = (a,b) => a + b // Could do the same thing as above, but on one line
```

## setTimeout & setInterval

- setTimeout
  - Works similar to sleep in Python
- setInterval
  - Lets you set a function to run at a specific interval (every 3 seconds, every 5 minutes, etc)
  - Sets an id
  - Usually you would set the setInterval method to variable
    - To stope the interval, you would call the clearInterval() method with the above variable as an argument

## The filter Method

- Creates a new array with all elemetns that pass the test implemented by the provided function
- Almost like an "if" in function form for an array

```Javascript
const nums = [9,8,7,6,5,4,3,2,1];
const odds = nums.filter(n => {
    return n % 2 === 1; //our callback returns true or false
    //if it returns true, n is added to the filtered array
})

const smallNums = nums.filter(n => n < 5);

```

## Some & Every Methods

- Every
  - Tests whether all elements in the array pass the provided function. Returns a boolean value
- Some
  - Similar to every, but returns true if ANY of the array elements pass the test function

## The Reduce Method

- Executes a reducer function on each element of the array, resulting in a single value

```Javascript
// Summing an array
[3,5,7,9,1].reduce((accumulator, currentValue) => {
    return accumulator + currentValue;
});
```

| Callback    | accumulator | currentValue | return value |
| ----------- | ----------- | ------------ | ------------ |
| first call  | 3           | 5            | 8            |
| second call | 8           | 7            | 15           |
| third call  | 15          | 9            | 24           |
| fourth call | 24          | 11           | 35           |

## Arrow Functions & "This"

- Arrow functions will not work with "this" depending on the execution context
