package day9

import (
	"os"
	"strings"
)

func getInput() ([][]int, error) {
	dat, err := os.ReadFile("./inputs/9.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	grid := make([][]int, len(lines))
	for row, l := range lines {
		rl := []rune(l)
		grid[row] = make([]int, len(rl))
		for col, v := range rl {
			grid[row][col] = int(v - '0')
		}
	}

	return grid, nil
}
