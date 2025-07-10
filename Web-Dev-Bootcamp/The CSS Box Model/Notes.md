# The CSS Box Model

- The theory that everything in CSS is a box
- See Box_Model_and_Units directory for examples

## Width and Height

- Width sets the width of the content area
- Height sets the height of the content area

## Border and Border-Radius

- Border is the border around an element
  - Has a lot of related attributes such as:
    - border-width
      - Controls the thickness of the border
    - border-height
    - border-color
      - Controls the color of the border
    - border-style
      - Controls the line style - dashed, solid, etc
    - border-radius
    - etc
  - Border adds to the size of the element
- Border-Radius
  - Rounds the corners of an element's outer border edge
  - Can use pixel, percentage, em, rem, etc values
    - Percentage is based on your width and height
    - Check the docs for the different values of border-radius

## Padding

- Padding is the area between the content in an element and the border of the box
- You can set individual properties such as:
  - padding-left
  - padding-right
  - padding-top
  - padding-bottom
- More commonly, you can set all four using the padding property
  - This can be done by adding 4 "sizes" of padding, see Box_Model > app.css
  - 4 ways of using this:
    - Apply to all 4 sides:
      - padding: 10px
    - vertical | horizontal
      - padding: 5px 10px
    - top | horizontal | bottom:
      - padding: 1px 2px 2px
    - top | right | bottom | left
      - padding: 5px 1px 0 2px

## Margin

- The space outside of an element's border
  - Essentially the space between one element and another
- Can be set using the same syntax as padding above whether individually or the margin overall
- The body element has a margin of 8px set by default
  - A lot of developers remove that so that the content goes all the way across the page

## The Display Property

- Can be used to set how an element is displayed such as in-line, block, and inline-block
  - Inline:
    - Width and Height are ignored. Marging and padding push elements away horizontally but not vertically
  - Block:
    - Block elements break the flow of a document. Width, Height, Margin, and padding are respected
  - Inline-block:
    - Behaves like an inline element except Width, Height, Margin, and padding are respected

## CSS Units Revisited

- Percentages
- Percentages are always relative to some other value
- Sometimes, it's a value from the parent and other times it's a value from the element itself
  - width: 50% - half the width of the parent
  - line-height: 50% - half the font size of the element itself

### ems

- Relative Units
  - With font-size, 1em equals the font-size of the parent. 2ems is twice the font-size of the parent
  - With other properties, 1em is equal to the computed font-size of the element itself

### rems

- Relative Units
  - Root Ems
    - Relative to the root html element's font-size. Often easier to work with
    - If the root font-size is 20px, 1rem is always 20px
