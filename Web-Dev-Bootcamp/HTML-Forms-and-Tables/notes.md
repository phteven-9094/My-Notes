# Forms and Tables

## HTML Tables

- Structured sets of data, made up of rows and columns.
- Created with the table element
- See heaviest_birds.html for examples

### Tables Elements: TR, TD, and TH

- TR Element
  - Table Row
  - Represents a row of cells in a table
- TD Element
  - Table Data Cell
  - Represent a single cell in a table
- TH Element
  - Table Headers
  - Represents headers of a table

### Tables: Thead, Tbody, Tfoot

- Thead
  - Wraps your row of headers to make it clear where your headers are
- Tbody
  - Wraps all rows that make up the body of a table
- Tfoot
  - Wraps your row of footers if you have one

### Tables: Colspan and Rowspan

- Colspan
  - Denotes how many columns you want a table element to take
- Rowspan
  - Denotes how many rows you want a table element to take

## The Form Element

- A shell or container that doesn't have any visual impact
- We then fill the form with a collection of inputs, checkboxes, buttons, etc
- Created with the form element
- Represents a document section containing interactive contorls for submitting information
- The action attribute specifies WHERE the form data should be sent
- The method attribute specifies which HTTP method should be use
- See forms.html for examples

### Common Input Types

- Input Element
  - 20+ possibly types of inputes, ranging from data pickers to checkboxes
  - The type attribute alters how the input element behaves
  - The placeholder attribute fills in the input element with what you want to hint to the user what it's for

### Labels

- Represents a caption for an item in a UI
- Generally linked with an input element
- Makes the label clickable when associated with another element
- Set an id to an input so you can notate what your label is for (associate the 2 elements)

### HTML Buttons

- Can create a button with the button element
- Can pass in the attribute type to change the type of button
  - Default is the submit type

### The Name Attribute

- A way of referring to the value of the form
- Essentially the name of the payload that is sent to the server
- Should be on every input that you use

### Radio Buttons, Checkboxes, and Selects

- Checkboxes are specified using type=checkbox in the input tag.
- They can also be set to be checked already or not
- Radio Buttons are specified using type=radio
- The value attribute is very important so you can send the value in the HTTP Request
- Selects are essentially a drop down menu and are specified using a select tag
- The selected attribute can be added to pre-select an option

### Range and Text Area

- Range is a slider like a volume slider
- Text Area is a multi-line text input and uses the textarea tag

## HTML5 Form Validations

- Consists adding constraints or validating user inputs have valid data
- The attribute required can be added to almost any element to require specific data
- Pattern can be used via Regex as well (won't be going over it for now)
- The URL type can also be used to validate a url is being entered
