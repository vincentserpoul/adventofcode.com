package day1

func Day12() (int, error) {
	measurements, err := getMeasurements()
	if err != nil {
		return 0, err
	}

	increased := 0
	prevWindow := measurements[0] + measurements[1] + measurements[2]
	for i := 3; i <= len(measurements)-1; i++ {
		currWindow := measurements[i] + measurements[i-1] + measurements[i-2]
		if currWindow > prevWindow {
			increased++
		}

		prevWindow = currWindow
	}

	return increased, nil
}
