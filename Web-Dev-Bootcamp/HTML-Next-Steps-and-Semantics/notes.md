# HTML Next Steps and Semantics

## What is HTML5?

- The latest version of the standard that defines HTML
- Includes new elements and features for browsers to implement

## Block vs. Inline Elements

- Inline Elements fit in alongside other elements
  - Examples:
    - Anchor Tags
    - Span Tags which is wrapped around any text or content that we want to single out
- Block elements take up a whole "block" of space
  - Examples:
    - div elements which is a generic container for flow content
    - p elements
- See chickens.html in HTML-The-Essentials for more examples

## Assortment of More Elements

- HR (horizontal rule) elements
  - Makes an horizontal line that is used as a divider between paragraph level elements
  - Can be styled by CSS
- BR (break) element
  - Makes a line break in your content
- sup (Superscript) element
  - Allows for superscript to be used such as exponents
- sub (subscript) element
  - Allows for subscript to be used

## Entity Codes

- Special sequences that can be used in HTML that will result in different characters
- Use Examples:
  - Used to display reserved characters, that normally would be invalid
  - Used in place of difficult to type characters
- Start with an ampersand and end with a semicolon
- The browser interprets them and renders the correct character instead

## Semantic Markup

- Semantic - Relating to meaning
  - "What purpose or role does that HTML element have?"
- Instead of Divs, use more specific elements like:
  - section
  - article
  - nav
  - main
  - header
  - footer
  - aside
  - summary
  - details
