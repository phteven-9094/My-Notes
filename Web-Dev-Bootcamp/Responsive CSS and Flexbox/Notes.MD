# Responsive CSS and Flexbox

## What is Flexbox

- A one-dimensional layout method for laying out items in rows or columns
- Allows us to distribute space dynamically across elements of an unknown size
- Look at examples in the Flexbox and Media Queries folders
- Made up of 2 axis'
  - Main goes from left to right
  - Cross goes up and down
- Always check docs for more info!

## Flex-Direction

- Used with Display: flex
  - This essential "turns on" flexbox" when set on an element
- flex-direction lets us decide on the main axis direction and defaults to row

## Justify-Content

- Determines how the content is distributed across the main axis
- Defaults to flex-start
- Options include start, end, center, space-between, space-around, etc

## Flex-Wrap

- Determines whether elements are going to wrap along the main axis in different ways such as new line, column, etc

## Align-Items

- Distributes items across the cross axis

## Align-Content and Align-Self

- Align-content:
  - Control or distribute space across the cross-axis but only if there are multiple columns or rows
  - Applies to single elements in an container

## Flex-Basis, Grow, and Shrink

- flex-basis
  - Defines the initial size of an element before additional space is distributed.
- flex-grow
  - Controls the amount of available space an element should take up. Accepts a unit-less number value
- flex-shrink
  - If items are larger than the container, they shrink according to flex-shrink

## Flex Shorthand

- Can use just flex that sets flex-basis, grow, and shrink in the order of flex-grow | flex-shrink | flrx-basis
- Check the docs for more options for the shorthand

## Responsive Design and Media Queries

- Can use Media Queries to let a page respond to a screen size
- For example:
  - If element is 500px, make it green
  - If element is more than 1000px, but less than 1500, make it yellow
  - etc
