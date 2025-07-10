# Newer JavaScript Features

- Crucial Information
  - Spread with Arrays
  - Spread with Objects
  - Destructuring
- Important Information
  - Default Params
  - Spread in Function Calls
  - Rest Params

## Default Params

- You can set the default paramater by assigning it in the parameter list
- The default will run if the user does not enter in the parameter
  - If they do pass in the parameter, it will use that

```Javascript
function multiply(a, b = 1) {
    return a * b;
}
multiply(4); //4
multiply(4, 5); //20
```

## Spread in Function Calls

- Spread syntax allows an iterable such as an array to be expanded in places where zero or more arguments (function calls) or elements (for array literals) are expected, or an object expression to be expanded in places where zero or more key-value pairs (for object literals) are expected.
- Essentially, it takes an iterable and makes each element of the iterable into it's own argument
- When spreading arrays and objects, it will create a new array and object rather than modifying the original
- Check the Spread directory for examples

## Rest Params

- Seems to work similar to \*args in Python while looking similar to spread

```Javascript
function sum(...nums) {
    return nums.reduce((total, el) => total + el)
}
```

## Destructuring

- A short, clean syntax to 'unpack':
  - values from arrays
  - properties from objects
- into distinct variables

## Destructuring Arrays

```Javascript
const raceResults = ['Eliud Kipchoge','Feyisa Lelisa','Galen Rupp'];

const [ gold, silver, bronze ] = raceResults;
gold; //'Eliud Kipchoge'
silver; // 'Feyisa Lelisa'
bronze; // 'Galen Rupp'

const [ fastest, ...everyoneElse ] = raceResults;
fastest; // 'Eliud Kipchoge'
everyoneElse; // ['Feyisa Lelisa','Galen Rupp']
```

## Destructuring Objects

- See destructuring directory

## Destructuring Params

- See destructuring directory
