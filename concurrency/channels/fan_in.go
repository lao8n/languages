package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	// producers
	ch := make(chan string)
	wg := sync.WaitGroup{}
	messageList := [][]string{
		{"hi", "from", "the", "thread"},
		{"more", "messages", "for", "you"},
	}
	for _, list := range messageList {
		wg.Add(1)
		go producer(ch, &wg, list)
	}

	// close channel
	go func() {
		wg.Wait()
		close(ch)
	}() // avoids deadlock because not waiting to receive before closing channel

	// receive
	for received := range ch {
		fmt.Println("Got: ", received)
	}
}

func producer(out chan<- string, wg *sync.WaitGroup, messages []string) {
	go func() {
		defer wg.Done()
		for _, m := range messages {
			out <- m
			time.Sleep(time.Second * 1)
		}
	}()
}
