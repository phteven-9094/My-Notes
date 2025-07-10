# Other Assorted Useful CSS Properties

- Examples can be found in their respective folder under the Other_Properties folder

## Opacity and the Alpha Channel

- Alpha Channel refers to a 4th value in RGB that determines transparency
  - Ranges from 0 to 1 where 1 is not transparent and 0 is completely transparent
  - Only affects the specifc property being adjusted of the element
- Opacity is a property that is set on an element and determines it's transparency
  - Includes the entire element and any descendants

## The Position Property

- Sets how an element is positioned in a document. The top, right, bottom, and left properties determine the final location of positioned elements.
- Static
  - Default value
  - The top, right, bottom, and left have no affect if position is set to static
- Relative
  - Element is positioned according to the normal flow of the document, and then it can be offest using top, right, bottom, and left
- Absolute
  - Element is removed from the document flow and no space is created for the element in the page layout.
  - Positioned relative to its closest positioned ancestor
- Fixed
  - Element is positioned relative to the initial contianing block
  - Can be used to create things like navbars that stay at the top of the screen while the page is scrolled

## CSS Transitions

- Can be used to change the behavior of elements such as transition a square into a circle over time on hover
- Transition: Property Name | Duration | Timing Function | Delay
  - If you want all properties, you can specify that with all
  - Timing functions can affect how the transition works such as ease-in or ease-out
    - There's a lot of timing functions; Use the docs for more info on transitions!

## CSS Transforms

- Allows you to change an element such as rotation, scaling, skewing, changes in perspective, etc
- Check the docs for more info!

## GoogleFonts and More Background

- Check the GoogleFonts and Background folders under Other Properties
