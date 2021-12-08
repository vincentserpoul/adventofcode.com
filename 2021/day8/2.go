package day8

func Day82() (int, error) {
	ps, err := getInput()
	if err != nil {
		return 0, err
	}

	count := 0
	for _, p := range ps {
		count += p.Display()
	}

	return count, nil
}
