package day9

func Day91() (int, error) {
	grid, err := getInput()
	if err != nil {
		return 0, err
	}

	sumRisk := 0
	for row, line := range grid {
		for col, val := range line {
			if isLowPoint(grid, row, col) {
				sumRisk += val + 1
			}
		}
	}

	return sumRisk, nil
}

func isLowPoint(grid [][]int, row, col int) bool {
	currV := grid[row][col]
	around := [][]int{
		{row - 1, col},
		{row, col + 1},
		{row + 1, col},
		{row, col - 1},
	}

	for _, c := range around {
		if c[0] < 0 || c[0] > len(grid)-1 || c[1] < 0 || c[1] > len(grid[0])-1 {
			continue
		}
		if currV >= grid[c[0]][c[1]] {
			return false
		}
	}

	return true
}
