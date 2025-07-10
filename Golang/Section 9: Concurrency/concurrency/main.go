package main

import (
	"fmt"
	"time"
)

func greet(phrase string, doneChan chan bool) {
	fmt.Println("Hello!", phrase)
	doneChan <- true
}

func slowGreet(phrase string, doneChan chan bool) {
	time.Sleep(3 * time.Second) // simulate a slow, long-taking task
	fmt.Println("Hello!", phrase)
	doneChan <- true // Essentially returns the bool True
	close(doneChan)
}

func main() {
	done := make(chan bool)
	// dones := make([]chan bool, 4)
	//done := make(chan bool)
	// dones[0] = make(chan bool)
	go greet("Nice to meet you!", done) //dones[0])
	// dones[1] = make(chan bool)
	go greet("How are you?", done) //dones[1])
	// dones[2] = make(chan bool)
	go slowGreet("How ... are ... you ...?", done) //dones[2])
	// dones[3] = make(chan bool)
	go greet("I hope you're liking the course!", done) //dones[3])
	// <-done //Essentially sets the channel/routine to be done
	// for _, done := range dones {
	// <-done
	// }

	for range done {
		<-done
	}

}
