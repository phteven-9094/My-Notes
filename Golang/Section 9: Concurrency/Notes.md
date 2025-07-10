# Concurrency - Running Tasks in Parallel

## Introducing Goroutines

- Concurrency in Go
  - Running code in parallel
- Goroutines are a way to use concurrency in Go

## Running Functions as Goroutines

- To run functions in parallel, you simply add the "go" keyword before a function
- You have to run a goroutine in a non-blocking way
- If you have several functions back to back, it won't necessarily wait for each one to finish, so it won't return anything

## Introducing & Using Channels

- To remedy the above problem, channels are used
- First, you should create a channel with the make keyword
  - See done in main.go
- Then, pass the channel to the function that you want to run as a goroutine
- When you pass the channel into a function, you need to show the data that is passing into the channel with the <- keyword. See slowGreet() in main.go

## Working with Multiple Channels & Goroutines

- You can send multiple values from different goroutines through the same channel
- You will likely run into race condition using channels
- If you have one channel across multiple goroutines, you can create a slice and either increment it or store new elements in the slice
  - See dones in main.go
- You can use a for loop, but personally, I think I like the slice concept more

## Goroutines & Channels in a Project

- Check priceCalcconcurrency directory

## Setting up An Error Channel

- Check priceCalcconcurrency directory ("errChan")

## Managing Channels with the "select" Statement

- Control structure that is specifically created to be used with channels
- You can essentially create a switch statement to wait for 2 or more channels to emit a value

## Deferring Code Execution with "Defer"

- You can use the defer keyword to setup default behavior when a function hits a breakpoint
- Check filemanager.go
