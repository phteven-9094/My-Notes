# Working with Packages

## Splitting Code Across Files in the Same Package

- You might want to split code across files for readability and ease of modifying the code
- You can create a new file, you just have to add the package that the code belongs to
  - For example, main in the bank.go file

## Preparing Code for Multiple Packages

- See the bank app to see how it changed from Section 1

## Exporting and Importing Identifiers (Variables, Functions, & More)

- If you want to export something, it must start with a capital letter
- Anything that starts with lower-case letters, will be private to the package

## Using Third-Party Packages

- You'll want to search for a third party package if you need to use one
- Similar to how you import your own package, you need to import it with a site name such as:
  - Github.com/Pallinder/go-randomdata
  - You just need to install it first with the go get command
- If you use go get, it will check your go.mod file and install all dependencies
