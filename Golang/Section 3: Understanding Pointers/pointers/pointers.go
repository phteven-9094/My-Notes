package main

import "fmt"

func main() {
	age := 32 //regular variable

	// var agePointer *int
	// agePointer = &age
	agePointer := &age

	fmt.Println("Age:", *agePointer)
	fmt.Println(getAdultYears(agePointer))

}

func getAdultYears(age *int) int { //to return the pointer *age, you would delete the expected return int declaration
	return *age - 18
	//*age = *age - 18
}
