package main

import (
	"fmt"
	"github.com/jhungerford/adventofcode-2021/internal/day20"
)

func main() {
	puzzle, err := day20.LoadPuzzle("input/day20.txt")
	if err != nil {
		fmt.Printf("Failed to load Puzzle: %v", err)
		return
	}

	fmt.Println("Part 1: ", puzzle.Run(2).NumLit())
	fmt.Println("Part 2: ", puzzle.Run(50).NumLit())
}
