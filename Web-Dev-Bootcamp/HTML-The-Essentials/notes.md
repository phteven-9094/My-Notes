# The Essentials of HTML

## Introduction to HTML

- HTML is the content on the page
  - CSS is the design of the page
  - Javascript is the actinos of the page
- The goal of HTML is to take text content and mark it up so it has structure
- HTML Elements
  - To write HTML, we pick from a set of standard elements that all browsers recognize
    - These are represented by HTML tags such as
      - p for paragraphs
      - h1 for main headers
      - img for images
    - Tags are all opened and closed with <></>

## Our First HTML Page

- See chickens.html

## Paragraph Elements

- Represented with p tags
- Takes everything between the tags and makes it it's own separate element.

## Heading Elements

- Represented by h1 through h6 tags
  - h1 through h3 are the most widely used with h4 through h6 used rarely
- Used to seperate headings on a page
  - There should only be one h1 element on any given page
- Don't use heading elements for size, but to show levels of importance

## HTML Boilerplate

- Provides a skeleton of sorts for your html document
- Can be auto-created by typing html:5 and hitting tab or ! then tab
- There should be one head and one body
  - The head is the document metadata
    - The information is info about the document such as CSS stylesheets, assets, and javascript scripts, but is not displayed in the browser
    - The title element is what you see displayed in the browser tab of the page and is located under the head element
  - The body element consists of everything on the page

## List Elements

- 2 types:
- Bulleted (Unordered)
  - Represented by ul tag
  - Consists of li tags within the elements to show a single item in the list
- Numbered (Ordered)
  - Represented by ol tag
  - Ordered lists also consist of li tags
- Unordered and Ordered lists can be nested and can also have other elements such as bold, italic, etc

## Anchor Tags

- Creates a hyperlink to web pages, files, email addresses, etc
- Represented by the a tag
- Every anchor tag uses the href attribute

## Images

- Adds an image to a page
- Represented by the img tag
- Attributes that are used in the img element include:
- class
  - "Title" that you can give the image
- src
  - Source URL of the image
- alt
  - Alternative text that appears when you hover over the image as a brief description of the image

## Comments

- To leave comments on a HTMl page, see the comment after the h1 header in chickens.html
