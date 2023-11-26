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

	part1 := puzzle.Run(2)
	fmt.Println("Part 1: ", part1.NumLit())
}
