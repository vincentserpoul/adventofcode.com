package day2part2

func Day22() (int, error) {
	cmds, err := getCommands()
	if err != nil {
		return 0, err
	}

	pos, depth, aim := 0, 0, 0
	for _, c := range cmds {
		pos, depth, aim = ApplyCommand(pos, depth, aim, c)
	}

	return pos * depth, nil
}
