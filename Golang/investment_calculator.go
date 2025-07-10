package main

import (
	"fmt"
	"math"
)

const inflationRate = 2.5

func main() {

	var investmentAmount float64
	var years float64
	expectedReturnRate := 5.5
	// var years float64 = 10
	// Alt way
	// investmentAmount, years, expectedReturnRate := 1000.0, 10.0, 5.5

	// fmt.Print("Investment Amount: ")
	outputText(("Investment Amount: "))
	fmt.Scan(&investmentAmount)

	//fmt.Print("Expected Return Rate: ")
	outputText(("Expected Return Rate: "))
	fmt.Scan(&expectedReturnRate)

	//fmt.Print("Number of Years: ")
	outputText(("Number of Years: "))
	fmt.Scan(&years)

	//futureValue := investmentAmount * math.Pow(1+expectedReturnRate/100, years)
	//futureRealvalue := futureValue / math.Pow(1+inflationRate/100, years)

	// formattedFV := fmt.Sprintf("Future Value: %.2f\n", futureValue)
	// formattedRFV := fmt.Sprintf("Future Value (adjusted for Inflation): %.2f\n", futureRealvalue)
	// fmt.Println(futureValue)
	//fmt.Printf("Future Value: %.2f\nFuture Value (adjusted for Inflation): %.2f\n", futureValue, futureRealvalue)
	// fmt.Println(futureRealvalue)
	//fmt.Print(formattedFV, formattedRFV)
	//fmt.Printf(`Future Value: %.2f\n
	//Future Value (adjusted for Inflation): %.2f\n`, futureValue, futureRealvalue)

	futureValue, futureRealvalue := calculateFutureValue(investmentAmount, expectedReturnRate, years)
	fmt.Print(futureValue, futureRealvalue)

}

func outputText(text string) {
	fmt.Print(text)

}

// func calculateFutureValue(investmentAmount float64, expectedReturnRate float64, years float64) (float64, float64) {
// 	fv := investmentAmount * math.Pow(1+expectedReturnRate/100, years)
// 	rfv := fv / math.Pow(1+inflationRate/100, years)
// 	return fv, rfv
// }

func calculateFutureValue(investmentAmount float64, expectedReturnRate float64, years float64) (fv float64, rfv float64) {
	fv = investmentAmount * math.Pow(1+expectedReturnRate/100, years)
	rfv = fv / math.Pow(1+inflationRate/100, years)
	return
}
