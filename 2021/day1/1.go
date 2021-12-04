package day1

func Day11() (int, error) {
	measurements, err := getMeasurements()
	if err != nil {
		return 0, err
	}

	increased := 0
	prev := measurements[0]

	for _, val := range measurements[1:] {
		if val > prev {
			increased++
		}
		prev = val
	}

	return increased, nil
}
