package main

import (
	"fmt"
	"time"
)

// TODO: under the hood channel is a buffer with mutexes to manage access
func main() {
	// golang channels default un-buffered and block the goroutine but not the underlying thread
	// uses m:n scheduling
	ch := make(chan string)
	go func() {
		defer close(ch)
		messages := []string{"hi", "from", "the", "thread"}
		for _, m := range messages {
			ch <- m // golang copies whether it is value or reference
			time.Sleep(time.Second * 1)
		}
	}()

	for received := range ch {
		fmt.Println("Got: ", received)
	}
}
