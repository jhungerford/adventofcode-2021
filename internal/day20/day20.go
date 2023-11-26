package day20

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"path"
)

// Position is a row and column in a Grid.
type Position struct {
	row int
	col int
}

// Grid contains pixels that are lit on an infinite Grid, as well as the value for pixels that aren't tracked.
type Grid struct {
	pixels      map[Position]bool
	otherPixels bool
}

// Puzzle contains an image enhancement algorithm and infinite Grid.
type Puzzle struct {
	enhancement []bool
	grid        Grid
}

// fromPixel converts a pixel to a byte suitable for printing.
func fromPixel(lit bool) string {
	if lit {
		return "#"
	}

	return "."
}

// toPixel converts a byte to a pixel.
func toPixel(b byte) bool {
	return b == '#'
}

// LoadPuzzle loads a Puzzle from the given inputFile.  The file contains a 512-byte image enhancement algorithm on
// the first line, a blank line, and a Puzzle Grid.
func LoadPuzzle(inputFile string) (Puzzle, error) {
	pwd, err := os.Getwd()
	if err != nil {
		return Puzzle{}, fmt.Errorf("failed to get working dir: %w", err)
	}

	file, err := os.Open(path.Join(pwd, inputFile))
	if err != nil {
		return Puzzle{}, fmt.Errorf("failed to open file: %w", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	// First line is the enhancement
	if !scanner.Scan() {
		return Puzzle{}, errors.New("missing enhancement line")
	}

	var enhancement []bool
	for _, b := range scanner.Bytes() {
		enhancement = append(enhancement, toPixel(b))
	}

	p := Puzzle{
		enhancement: enhancement,
		grid: Grid{
			pixels:      map[Position]bool{},
			otherPixels: false,
		},
	}

	// Second line is blank
	if !scanner.Scan() {
		return Puzzle{}, errors.New("enhancement and Puzzle should be separated by a blank line")
	}

	// Remaining lines are the pixels
	row := 0
	for scanner.Scan() {
		for col, b := range scanner.Bytes() {
			p.grid.set(row, col, toPixel(b))
		}

		row++
	}

	return p, nil
}

// Run applies the image enhancement algorithm the given number of times, returning a new Grid.
func (p Puzzle) Run(steps int) Grid {
	g := p.grid

	for i := 0; i < steps; i++ {
		g = g.step(p.enhancement)
	}

	return g
}

// String satisfies the Stringer interface, allowing puzzles to be printed.
func (p Puzzle) String() string {
	s := ""

	for _, lit := range p.enhancement {
		s = fmt.Sprintf("%s%s", s, fromPixel(lit))
	}

	s = fmt.Sprintf("%s\n\n%s", s, p.grid)

	return s
}

// step applies the image enhancement algorithm to the Grid once, returning a new Grid.
func (g Grid) step(enhancement []bool) Grid {
	// Other pixels outside the edge of the old grid share the same value.
	otherIndex := 0
	if g.otherPixels {
		otherIndex = 255
	}

	newGrid := Grid{
		pixels:      map[Position]bool{},
		otherPixels: enhancement[otherIndex],
	}

	// Expand the tracked Grid - pixel values on the edge of the old Grid can change.
	gridMin, gridMax := g.bounds()

	for row := gridMin.row - 1; row <= gridMax.row+1; row++ {
		for col := gridMin.col - 1; col <= gridMax.col+1; col++ {
			newGrid.set(row, col, enhancement[g.getEnhancementIndex(row, col)])
		}
	}

	return newGrid
}

// NumLit returns the number of pixels that are lit on this Grid.
func (g Grid) NumLit() int {
	count := 0

	for _, lit := range g.pixels {
		if lit {
			count++
		}
	}

	return count
}

// set whether the pixel is lit at the given Position.
func (g Grid) set(row, col int, lit bool) {
	g.pixels[Position{row: row, col: col}] = lit
}

// setOthers sets whether other pixels are lit on this infinite Grid.
func (g Grid) setOthers(lit bool) {
	g.otherPixels = lit
}

// get returns whether the pixel at the given Position is lit.
func (g Grid) get(row, col int) bool {
	lit, ok := g.pixels[Position{row: row, col: col}]
	if ok {
		return lit
	}

	return g.otherPixels
}

// getEnhancementIndex looks at a 3x3 square of pixels around the given Position, and converts it to an index in the
// image enhancement algorithm.
func (g Grid) getEnhancementIndex(row, col int) int {
	code := 0

	for plusRow := -1; plusRow <= 1; plusRow++ {
		for plusCol := -1; plusCol <= 1; plusCol++ {
			code <<= 1

			if g.get(row+plusRow, col+plusCol) {
				code += 1
			}
		}
	}

	return code
}

// bounds returns the minimum row/column and maximum row/column for pixels that are tracked on this Grid.
func (g Grid) bounds() (gridMin, gridMax Position) {
	for pos, _ := range g.pixels {
		gridMin.row = min(gridMin.row, pos.row)
		gridMin.col = min(gridMin.col, pos.col)
		gridMax.row = max(gridMax.row, pos.row)
		gridMax.col = max(gridMax.col, pos.col)
	}

	return gridMin, gridMax
}

// String converts this Grid to a String that can be printed.
func (g Grid) String() string {
	str := ""

	gridMin, gridMax := g.bounds()

	for row := gridMin.row; row <= gridMax.row; row++ {
		for col := gridMin.col; col <= gridMax.col; col++ {
			str = fmt.Sprintf("%s%s", str, fromPixel(g.get(row, col)))
		}

		str += "\n"
	}

	str = fmt.Sprintf("%sother pixels: %s", str, fromPixel(g.otherPixels))

	return str
}
