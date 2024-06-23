package main

import (
	"fmt"
	"time"
)

func main() {
	ch := make(chan string)
	go func() {
		defer close(ch)
		messages := []string{"hi", "from", "the", "thread"}
		for _, m := range messages {
			ch <- m
			time.Sleep(time.Second * 1)
		}
	}()

	for received := range ch {
		fmt.Println("Got: ", received)
	}
}
