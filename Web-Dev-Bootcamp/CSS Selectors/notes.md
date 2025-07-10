# CSS Selectors

## Universal and Element Selectors

- Universal Selector
  - Selects everything with the \* symbol
- Element Selector
  - Select everything of a specific type of an html page (h1, h2, button, img, etc)
- Selector List
  - List all elements of a list (h1, h2 = select all h1s and h2s)

### The ID Selector

- Selects elements with a certain id attribute
- Begins with the # symbol
- See app.css for examples

### The Class Selector

- Selects elements of a certain class attribute
- Classes can be a group of elements so you can group some elements without styling all of them
- Begins with a .
  - .class

### The Descendant Selector

- Selects nested elements
- li a {}
  - Selects all a's that are nested inside an li

### The Adjacent and Direct-Descendant Selectors

- Adjacent Selector
  - Selects elements that are immediately after another element
  - Example
    - h1 + p {}
      - Select only the paragraphs that are immediately preceded by an h1
- Direct-Descendant Selectors
  - Selects only elements that are direct children of another element
  - Example
    - div > li {}
      - Select only the li's that are direct childen of a div element

### The Attribute Selector

- Allows you to select elements based on a specific attribute
- Example
  - input[type="text"] {}
    - Select all input elements where the type attribute is set to text

## Pseudo Classes

- A keyword that's added to a selector that specifies a special state of the selected element(s)
- Examples:
  - Active
  - Checked
  - First
  - first-child
  - hover
  - not()
  - nth-child()
  - nth-of-type()

## Pseudo Elements

- A keyword added to a selector that lets you style a particular part of selected elements:
  - ::after
  - ::cue
  - ::first-letter
  - Check MDN docs for more examples

## The CSS Cascade

- The order your styles are declared in and linked to matters
- If 2 styles are made of the same kind, like 2 h1s, the first h1 will be styled, but when it hits the next h1, it will be styled to the new one
- If you link 2 stylesheets, the 2nd stylesheet will ultimately be implemented due to overwriting the first one

## Specificity

- How the browser decides which rules to apply when there's a conflict
- It is a measure of how specific a given selector is. The more specific selector "wins"
- Example
  - An element selector + an element selector is more specific than just an element selector
- General formula
  - !Important > Inline Styles > ID > Class > Element
- If you mark a style important, it marks it as the most specific thing and overrides all declarations
- Important and Inline styles should very rarely to never be used
