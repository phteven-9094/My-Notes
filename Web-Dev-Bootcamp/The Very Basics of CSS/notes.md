# The Very Basics of CSS

## What is CSS?

- Cascading Stle Sheets
- A way to style an html page
- The adjectives of a page
- A language for describing how documents are presented visually - how they are arranged and styled
- CSS is massive; use https://developer.mozilla.org/en-US/docs/Web/CSS/Reference

## Including Styles Correctly

- Inline Styles:
  - You can write your styles directly inline on each element, but this is not a good idea most of the time
- The Style Element
  - You can write your styles inside of a style element. This is easy, but it makes it impossible to share styles between documents. Not recommended.
- External Stylesheet
  - Write your files in a .css file, and then include them using a link in the head of your html document. Recommended!
- See examples in index.html and app.css

## Color and Background Color Properties

- The color property changes the text color
- background-color changes the color of the background of an element

### RGB and Named Colors

- Browsers recognize 140 different named characters
- You can specify RGB values to assign a color as well
- You can also specify Hexidecimal values to assign a color

## Common Text Properties

- text-align
  - Determines how text aligns
  - Defaults to center
- font-weight
  - Controls the boldness or lightness of text
    - 400 is normal and 700 is the usual bold (if you use numbers to determine the font weight)
- text-decoration
  - Sets the appearance of decorative lines on text (underlined, strikethrough, overlined etc) as well as it's style (dotted, wavy, dashed, etc)
- line-height
  - Controls the height of a line of text
- letter-spacing
  - Controls the amount of space between letters in pixels
- Check the text_properties folder for text and font examples

### Font Size Basics with Pixels

- Change the size of font with the font-size property
- Can be done in pixels, named values, em, and rem and more
- Pixels are absolute units and are not recommended for responsive websites
- Em and rem are relative units and will be covered later

### The Font Family Property

- Used to change the font of an element
- Custom fonts can be installed
- You will generally see a font stack which is a list of fonts that you want to use in order of desirability
