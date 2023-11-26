package day20_test

import (
	"fmt"
	"github.com/jhungerford/adventofcode-2021/internal/day20"
	"testing"
)

func TestSample(t *testing.T) {
	t.Parallel()

	puzzle, err := day20.LoadPuzzle("input/day20_sample.txt")
	if err != nil {
		t.Fatalf("failed to load Puzzle: %v", err)
	}

	fmt.Println(puzzle)

	grid := puzzle.Run(2)

	if grid.NumLit() != 35 {
		t.Fatalf("Grid should have 35 pixels after 2 steps, but has %d\nGrid:\n%s", grid.NumLit(), grid)
	}
}
