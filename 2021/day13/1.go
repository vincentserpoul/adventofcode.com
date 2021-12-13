package day13

func Day131() (int, error) {
	dots, instructions, err := getInput()
	if err != nil {
		return 0, err
	}

	res := applyInst(dots, instructions[0])
	displayGrid(res)
	return len(res), nil
}
