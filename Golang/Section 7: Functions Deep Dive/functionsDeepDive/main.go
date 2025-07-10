package main

import "fmt"

type transformFn func(int) int

func main() {
	numbers := []int{1, 2, 3, 4}
	moreNumbers := []int{5, 1, 2}
	doubled := transformNumbers(&numbers, double)
	tripled := transformNumbers(&numbers, triple)

	fmt.Println(doubled)
	fmt.Println(tripled)

	transformerFn1 := getTransformerFunction(&numbers)
	transfomerFn2 := getTransformerFunction(&moreNumbers)

	transformedNumbers := transformNumbers(&numbers, transformerFn1)
	moreTransformedNumbers := transformNumbers(&moreNumbers, transfomerFn2)

	fmt.Println(transformedNumbers)
	fmt.Println(moreTransformedNumbers)
}

// func doubleNumbers(numbers *[]int) []int {
// 	dNumbers := []int{}
// 	for _, value := range *numbers {
// 		dNumbers = append(dNumbers, double(value))

// 	}
// 	return dNumbers
// }

func transformNumbers(numbers *[]int, transform transformFn) []int {
	newNumbers := []int{}
	for _, val := range *numbers {
		newNumbers = append(newNumbers, transform(val))
	}
	return newNumbers
}

func getTransformerFunction(numbers *[]int) transformFn {
	if (*numbers)[0] == 1 {
		return double
	} else {
		return triple
	}
}

func double(number int) int {
	return number * 2
}

func triple(number int) int {
	return number * 3
}
