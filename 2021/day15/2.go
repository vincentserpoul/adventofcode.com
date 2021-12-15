package day15

func Day152() (int, error) {
	grid, err := getInput()
	if err != nil {
		return 0, err
	}

	return shortestPathRisk(grid, 5), nil
}
