# Interfaces & Generic Code

- Interfaces are used to guarentee that a method exists and is used including what they accept as arguments and what they return
- A common convention for naming interfaces if the interface only expects one method is to name it after that method + er
  - Example save() method would result in saver interface
- Structs are used to reduce repetitive code and is essentially a set of rules that a structs methods must abide
- An interface is like an HDMI port. You can find them on TVs or computer monitors. You'll also find them on PCs, Macs, set top boxes, Chromecasts, and a bunch of other stuff. All these things work differently, are made up of different parts, and are made by different manufacturers, but they all work together because they all use the HDMI interface. The interface means that no matter what the implementation (Chromecast, Mac, monitor), it will work.

## Creating a First Interface

- See saver interface in main.go

## Using the Interface

- See the saveData function in main.go

## Embedded Interfaces

- See outputtable interface in main.go

## The Special "Any Value Allowed" Type

- You can set an expected type in a function with interface{} or the any keyword

## Working with Type Switches

- See printSomething() in main.go

## Extracting Type Information From Values

- After a value, add a .()
  - Example
  - typedVal, ok := value.(int)
- Useful for testing and type checking

## Interfaces, Dynamic Types, and Limitations

- If you have a function accepting different types using any, you can use a check and then work with it:

  - ```Go
    func add(a, b interface{} interface{}) {
        aInt, aIsInt := a.(int)
        bInt, bIsInt := b.(int)

        if aIsInt && bIsInt {
            return aInt + bInt
        }
    }
    ```

- Generics can also be used for this, see next section

## Introducing Generics

- Generics are defined with []
- By using generics, Go will understand what the type should be

```Go
func add[T int|float64|string](a,b T) T { // T can be any type as long as it's an int, float, or string
    return a + b
}

```
