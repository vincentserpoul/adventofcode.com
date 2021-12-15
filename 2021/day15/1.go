package day15

func Day151() (int, error) {
	grid, err := getInput()
	if err != nil {
		return 0, err
	}

	return shortestPathRisk(grid, 1), nil
}
