package main

import "fmt"

// type Product struct {
// 	title string
// 	id    string
// 	price float64
// }

// type TemperatureData struct { This is unrealistic because you don't know how many days you want to track
// 	day1 float64
// 	day2 float64
// }

// func main() {
// 	var productNames [4]string = [4]string{"A Book"}
// 	prices := [4]float64{10.99, 9.99, 45.99, 20.00} // The brackets represent a list of values of a type. The number inside is how many items you're expecting in it. The values inside the {} are the items in the list
// 	fmt.Println(prices)

// 	productNames[2] = "A Carpet" //Target the third element and assign it a value
// 	fmt.Println(productNames)
// 	fmt.Println(prices[2])

// 	featuredPrices := prices[1:]
// 	highlightedPrices := prices[:3]
// 	fmt.Println(featuredPrices)
// 	fmt.Println(highlightedPrices)
// 	fmt.Println(len(featuredPrices), cap(featuredPrices))
// }

func main() {
	prices := []float64{10.99, 8.99}
	fmt.Println(prices[0:1])
	prices[1] = 9.99

	updatedPrices := append(prices, 5.99) //This creates a new array/slice and does not change the original
	fmt.Println(updatedPrices)

	discountPrices := []float64{101.99, 80.99, 20.59}
	prices = append(prices, discountPrices...)
	fmt.Println(prices)
}
