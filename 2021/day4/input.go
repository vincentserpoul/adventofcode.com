package day4

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type bingoGrid struct {
	grid             [][]int
	rowSelectedCount []int
	colSelectedCount []int
	valCellHM        map[int][2]int
	selected         map[int]struct{}
}

func (bg bingoGrid) sumNonSelected() int {
	sum := 0
	for _, row := range bg.grid {
		for _, cell := range row {
			if _, ok := bg.selected[cell]; !ok {
				sum += cell
			}
		}
	}

	return sum
}

func (bg *bingoGrid) draw(i int) (bool, error) {
	// check if it exists in the grid
	coords, ok := bg.valCellHM[i]
	if !ok {
		return false, nil
	}

	bg.selected[i] = struct{}{}
	bg.rowSelectedCount[coords[0]]++
	bg.colSelectedCount[coords[1]]++

	if bg.rowSelectedCount[coords[0]] == len(bg.grid) || bg.colSelectedCount[coords[1]] == len(bg.grid[0]) {
		return true, nil
	}

	return false, nil
}

func getInput() ([]*bingoGrid, []int, error) {
	dat, err := os.ReadFile("./inputs/4.txt")
	if err != nil {
		return nil, nil, err
	}

	lines := strings.Split(string(dat), "\n")
	// cut out the last one
	lines = lines[:len(lines)-1]

	// selection
	selectionS := strings.Split(lines[0], ",")
	selection := make([]int, len(selectionS))
	for i, v := range selectionS {
		var errConv error
		selection[i], errConv = strconv.Atoi(v)
		if errConv != nil {
			return nil, nil, fmt.Errorf("%s is not a number", v)
		}
	}

	// start at line 2
	lines = lines[1:]

	var gs []*bingoGrid
	var currGrid *bingoGrid
	var currRowIdx int
	l := 0

	for l <= len(lines)-1 {
		// detect if new grid (empty line)
		if len(lines[l]) == 0 {
			if currGrid != nil {
				gs = append(gs, currGrid)
			}

			currGrid = &bingoGrid{
				grid:             [][]int{},
				rowSelectedCount: []int{},
				colSelectedCount: []int{},
				valCellHM:        make(map[int][2]int),
				selected:         make(map[int]struct{}),
			}
			currRowIdx = 0
			l++
			continue
		}

		// if not, it's a new grid row to be appended to the current grid
		// parse the row
		rowS := strings.Fields(lines[l])
		row := make([]int, len(rowS))
		for i, v := range rowS {
			var errConv error
			row[i], errConv = strconv.Atoi(v)
			if errConv != nil {
				return nil, nil, fmt.Errorf("%s is not a number", v)
			}
		}

		// set the sum for this row
		currGrid.rowSelectedCount = append(currGrid.rowSelectedCount, 0)
		// add a row to the grid
		currGrid.grid = append(currGrid.grid, []int{})

		// fill the row in the grid/rowSelectedCount/colSelectedCount/valCellHM
		for colIdx, val := range row {
			currGrid.grid[currRowIdx] = append(currGrid.grid[currRowIdx], val)

			if currRowIdx == 0 {
				currGrid.colSelectedCount = append(currGrid.colSelectedCount, 0)
			}

			currGrid.valCellHM[val] = [2]int{currRowIdx, colIdx}
		}

		currRowIdx++
		l++
	}

	return gs, selection, nil
}
