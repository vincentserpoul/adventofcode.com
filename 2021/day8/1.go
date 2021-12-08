package day8

func Day81() (int, error) {
	ps, err := getInput()
	if err != nil {
		return 0, err
	}

	count := 0
	for _, p := range ps {
		count += p.Count1748()
	}

	return count, nil
}
