package day2part1

func Day21() (int, error) {
	cmds, err := getCommands()
	if err != nil {
		return 0, err
	}

	pos, depth := 0, 0
	for _, c := range cmds {
		pos, depth = ApplyCommand(pos, depth, c)
	}

	return pos * depth, nil
}
