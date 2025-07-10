# HTMX Essentials

## Using HTMX & Sending GET Requests

- Use the hx-get function
- You would just pass in the API endpoint that you want to issue the GET request
- hx-get="/"

## Controlling Response Content Rendering with hx-swap

- The hx-swap function lets you control how content gets rendered

## Defining Response Target with hx-target

- hx-target specifies the target element you want to update
- This takes a CSS selector as the attribute

## Changing the Request Trigger with hx-trigger

- hx-trigger controls how the request will be triggered
- You don't need hx-trigger if you want the request to happen on just a click
- For more triggers, modifiers, and variations
  - https://htmx.org/docs/#triggers

## From GET to POST with hx-post

- hx-post allows you to use a POST request
- The name attribute of an element will be how you can submit the request data
- HTMX will also follow redirects, which could potentially lead to elements being rendered with duplicates
- You may need to resend ONLY the updated HTML content with POST requests

## Picking Parts of a Response with hx-select

- An alternative to resending the HTML content, you can use hx-select
- The hx-select can be used to tell what part of the existing data should be updated
