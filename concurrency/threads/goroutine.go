package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	wg := sync.WaitGroup{}
	wg.Add(1)
	go func() {
		defer wg.Done()
		for i := 1; i < 10; i++ {
			fmt.Printf("hi number %d from the spawned thread\n", i)
			time.Sleep(time.Millisecond * 1) // blocks goroutine but frees up for other goroutines to use thread
		}
	}()
	for i := 1; i < 5; i++ {
		fmt.Printf("hi number %d from the main thread\n", i)
		time.Sleep(time.Millisecond * 1)
	}
	wg.Wait()
}
