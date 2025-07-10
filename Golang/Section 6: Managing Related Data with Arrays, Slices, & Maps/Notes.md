# Managing Related Data with Arrays, Slices, and Maps

## Introducing Arrays

- Arrays are a way to group similar pieces of data together
- Similar to Lists in Python
- See prices array in lists.go

## Working with Arrays

- You can select an element of an array with listName[number of element's position]

## Selecting Parts of Arrays with Slices

- A Slice is a way of selecting parts or a subset of an array
- You start with the element you want to start with
- Then, the 2nd value is where you want the slice to stop EXCLUDING the element you end at
- The third value is the step which is what sections you want to skip
- Ex.
  - array[1:3:2] //Start at element 1, end at element 3 excluding element 3, take 2 steps each time

## More Ways of Selecting Slices

- You can overwrite the orginal array by targeting the element and assigning it a new value
  - This does not copy the original array, it directly edits it
- You can view the length of an array with the len() function (built into Go)
  - This gives the number of elements in an array
- The cap() function will display the capacity for the array
  - The capacity is the max amount of items that can be in an array (this is also based on parent arrays)

## Building Dynamics: Lists With Slices

- Sometimes you won't know how many values you'll have in a list
- You can build a dynamic list with slices
  - First, declare an array with no capacity listed: prices := []float64

## Unpacking List Values

- Use the ... operator to unpack list values
- Check discountPrices in lists.go

## Introducing Maps

- Maps are another data structure
- Consists of key-value pairs
- Works similar to Python dictionaries
- Created with:
  - varName := map[keyType]valueType{}

## Mutating Maps

- Map values can be accessed with mapName["keyName"]
- Maps are always dynamic
- To create a new key:
  - mapName["newKeyName"] = "newValue"
- A key-value pair can be deleted with the delete() function
  - delete(mapName, "key")

## Using the Special "make" Function

- The make function will create a slice, and takes two arguments, the type and the initial amount of values
- Helpful if you want to target specific slots after array creation
- You can pass an optional third argument to make() for the capacity of the array

## "make"ing Maps

- Make can be used to create a map as well
  - make(map[keyType]valueType)

## Working With Type Aliases

## For Loops with Arrays, Slices, and Maps
