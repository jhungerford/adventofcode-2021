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

	for _, test := range []struct {
		steps    int
		expected int
	}{
		{2, 35},
		{50, 3351},
	} {
		grid := puzzle.Run(test.steps)

		actual := grid.NumLit()

		if actual != test.expected {
			t.Fatalf("Grid should have %d pixels after %d steps, but has %d\nGrid:\n%s",
				test.expected, test.steps, actual, grid)
		}
	}
}
