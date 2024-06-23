package main

import "fmt"

func main() {
	ch := make(chan []int)
	original := []int{1, 2, 3}

	go func() {
		ch <- original
		original[0] = 42 // This modification affects what is received
	}()

	received := <-ch
	fmt.Println(received[0]) // Output will be 42
}
