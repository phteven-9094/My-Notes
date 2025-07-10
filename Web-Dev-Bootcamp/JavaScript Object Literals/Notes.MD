# JavaScript Object Literals

## Introduction

- Seems very similar to Python dictionaries
- Objects are collections of properties
- Properties are a key-value pair
- Rather than accessing data using an index, we use custom keys

## Creating Object Literals

- Created using curly braces
- Can include any data types including arrays
- syntax:

```Javascript
const person = {
    firstName: 'Mick',
    lastName: 'Jagger'
}
```

## Accessing Data Out of Objects

- To access data in an Object, you use dot notation:

```Javascript
const person = {
    firstName: 'Mick',
    lastName: 'Jagger'
}

person.firstName
//You can access pieces similar to Python as well:
person["lastName"]
//You would use the key name if you have a variable
```

## Modifying Objects

- Call the key you want and then assign it the new value

```Javascript
const midterms = {danielle: 96, thomas: 78}
midterms.thomas = 79
midterms.ezra // Can add to the Object this way as well
midterms.ezra = "A-"
```

## Nesting Arrays & Objects

- Arrays and Objects can be used in conjunctions with each other:

```Javascript

// Example of an Array made up of Objects
const shoppingCart = [
    {
        product: "Jenga Classic",
        price: 6.88,
        quantity: 1,
    },
    {
        product: "Echo Dot",
        price: 29.99,
        quantity: 3,
    },
    {
        product: "Fire Stick",
        price: 39.99,
        quantity: 2
    }
]

//Examples of an object with a nested array and a nested object
const student = {
    firstName: "David",
    lastName: "Jones",
    strengths: ['Music', 'Art']
    exams: {
        midterm: 92,
        final: 88
    }
}



```
