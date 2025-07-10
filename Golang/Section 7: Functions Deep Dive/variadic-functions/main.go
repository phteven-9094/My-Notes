package main

import "fmt"

func main() {
	numbers := []int{1, 10, 5}
	// sum := sumup(&numbers)
	sum := sumup(1, 10, 15, 40, -5)
	fmt.Println(sum)
	anotherSum := sumup(1, numbers...)
	fmt.Println(anotherSum)
}

// func sumup(numbers *[]int) int {
// 	sum := 0

// 	for _, value := range *numbers {
// 		sum += value
// 	}
// 	return sum
// }

func sumup(startingValue int, numbers ...int) int {
	sum := 0
	fmt.Printf("The starting value is: %v", startingValue)

	for _, val := range numbers {
		sum += val
	}
	return sum
}
