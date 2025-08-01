package main

import "fmt"

// type Product struct {
// 	id    string
// 	title string
// 	price float64
// }

func main() {
	websites := map[string]string{
		"Google":              "https://google.com",
		"Amazon Web Services": "https://aws.com",
	}
	fmt.Println(websites)

	fmt.Println(websites["Amazon Web Services"])
	websites["LinkedIn"] = "https://linkedin.com" //Add new key-value
	fmt.Println(websites)
	delete(websites, "Google") // Deletes a key-value pair
}
