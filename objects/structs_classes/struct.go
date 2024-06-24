// go run struct.go

package main

import "fmt"

type Employee struct { // value type
	Id     int
	Name   string
	Stats  []int
	Jersey [1]int
}

func main() {
	// initialize struct
	e1 := Employee{
		Id:     3,
		Name:   "freeman",
		Stats:  []int{3, 4, 5},
		Jersey: [1]int{5},
	}

	e1.Id = 1
	e1.Name = "ohtani"
	e1.Stats[0] = 1
	e1.Stats[1] = 2
	e1.Stats[2] = 3
	e1.Stats = append(e1.Stats, 4)
	e1.Jersey[0] = 17

	// print
	fmt.Println(e1) // {1 ohtani [1 2 3 4] [17]}
}
