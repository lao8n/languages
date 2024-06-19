// go run struct.go
package main

import "fmt"

type Employee struct {
	Id     int
	Name   string // immutable utf-8 encoded
	Stats  []int
	Jersey [1]int
}

func deep_copy(e Employee) Employee {
	c := Employee{
		Id:     e.Id,
		Name:   e.Name,
		Stats:  make([]int, len(e.Stats)),
		Jersey: e.Jersey,
	}
	copy(c.Stats, e.Stats)
	return c
}

func main() {
	// initialize struct
	e1 := Employee{
		Id:     3,
		Name:   "freeman", // just a pointer to string
		Stats:  []int{3, 4, 5},
		Jersey: [1]int{5}, // deep copy
	}

	// deep copy
	e2 := deep_copy(e1)

	// shallow copy
	e3 := e1

	// update
	e1.Id = 1
	e1.Name = "ohtani"
	e1.Stats[0] = 1
	e1.Stats[1] = 2
	e1.Stats[2] = 3
	e1.Stats = append(e1.Stats, 4)
	e1.Jersey[0] = 17

	// print
	fmt.Println(e1) // {1 ohtani [1 2 3 4] [17]}
	fmt.Println(e2) // {3 freeman [3 4 5] [5]}
	fmt.Println(e3) // {3 freeman [1 2 3] [5]} - no 4
}
